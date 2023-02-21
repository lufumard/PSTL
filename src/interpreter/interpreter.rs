#![allow(dead_code)]

#[path = "../ast.rs"]
pub mod ast;

use std::collections::HashMap;
pub use std::primitive;
pub use crate::ast::Var;
pub use crate::ast::Expr;
pub use crate::ast::FnBody;
pub use crate::ast::Fn;
pub use crate::ast::AST;
pub use crate::ast::Const;
pub use crate::ast::CONST_FALSE;
pub use crate::ast::CONST_TRUE;
pub use crate::ast::CONST_NIL;
pub use crate::ast::CONST_LIST;


pub mod primitives;
use primitives::is_primitive;
use primitives::eval_fncall_primitive;


#[derive(Debug, Clone)]
pub enum Heap{
    Some {
        var:Var,
        value:Expr,
        next:Box<Heap>,
    },
    None,
}

pub fn start_interpreter (funs : Vec<AST>, exec: Expr, heap: &Heap) -> Expr {
    let mut liste_fun:HashMap<String, Fn> = HashMap::new();
    funs.iter().fold(Expr::Num(0), |_, f| {
        eval_ast(f.clone(), heap, &mut liste_fun)
    });
    //dbg!(&liste_fun);
    eval_expr(exec, heap, &mut liste_fun)
}

impl Heap {
    pub fn get(&self, v:Var) -> Expr{
        let Var::Var(nom1) = &v;
        match self {
            Heap::Some { var, value, next } => {  
                let Var::Var(nom2) = var;
                if *nom1 == *nom2 {
                    return value.clone();
                }
                return next.get(v);
            }
            Heap::None => panic!("{} pas dans le heap", nom1),
        }
    }

    pub fn add(&self, v:Var, val:Expr) -> Heap{
        return Heap::Some{
            var: v, 
            value: val, 
            next: Box::new(self.clone()),
        };
    }
}





pub  fn empty_heap() -> Heap {
    Heap::None
}

pub  fn make_false() -> Expr{
    return Expr::Ctor(CONST_FALSE, vec![]);
}

pub  fn make_true() -> Expr{
    return Expr::Ctor(CONST_TRUE, vec![]);
}

pub  fn make_nil() -> Expr{
    return Expr::Ctor(CONST_NIL, vec![]);
}

pub  fn make_list(args : Vec<Var>) -> Expr{
    return Expr::Ctor(CONST_LIST, args);
}

pub  fn throw() {
    panic!("Evaluation non défini pour ce type");
}

pub  fn get_nb_args_ctor(n: i32) -> i32 {
    match n {
        CONST_FALSE => 0,
        CONST_TRUE => 0,
        CONST_NIL => 0,
        CONST_LIST => 2,
        _ => panic!("Ctor {} non existant", n),
    }
}

pub  fn create_ctor(n: i32, args: Vec<Var>) -> Expr {

    match n {
        CONST_FALSE => make_false(),
        CONST_TRUE => make_true(),
        CONST_NIL => make_nil(),
        CONST_LIST => make_list(args),
        _ => panic!("Contructeur non connu"),
    }
}

// Ast evaluation
pub  fn eval_ast(ast: AST, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    match ast {
        AST::Fn(fun) => eval_fun(fun, h, lfn),
        AST::Var(var) => eval_var(var, h, lfn),
        AST::Expr(expr) => eval_expr(expr, h, lfn),
        AST::FnBody(body) => eval_fnbody(body, h, lfn),
        AST::Const(c) => eval_const(c, h, lfn),
        AST::Program(c, fun) => eval_program(c, fun, h, lfn),
    }
}

/*
* Var evaluation section
*/
pub fn eval_var(var: Var, h: &Heap, _:&mut HashMap<String,Fn>) -> Expr {
    h.get(var)
}

/*
* Expr evaluation section
*/

pub  fn eval_expr(expr: Expr, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    match expr {
        Expr::FnCall(ident, vars) => eval_fncall(ident, vars, h, lfn),
        Expr::PartialFnCall(x, y) => eval_pap_fncall(x, y, h, lfn),
        Expr::Pap(ident, vars) => eval_pap(ident, vars, h, lfn),
        Expr::Ctor(n, vars) => eval_ctor(n, vars, h, lfn),
        Expr::Proj(n, var) => eval_proj(n, var, h, lfn),
        Expr::Num(n) => eval_value(n, h, lfn),
    }
}

pub  fn eval_fun(fun:Fn, _: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    let Fn::Fn(cst, _, _) = fun.clone(); 
    let Const::Const(name) = cst;
    lfn.insert(name.clone(), fun);
    make_false()
}


pub fn eval_fncall(ident: Const, vars: Vec<Var>, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    let Const::Const(nom) = ident;
    if is_primitive(&nom) {
        return eval_fncall_primitive(nom, vars, h, lfn);
    }

    match lfn.get(&nom).cloned() {
        Some(Fn::Fn(ident, args, body)) => {
            if args.len() == vars.len() {
                eval_cons_full_fn(args, body, vars, h, lfn)
            } else {
                if args.len() > vars.len() {
                    eval_pap(ident, vars, h, lfn)
                } else {
                    panic!("{} n'a pas autant d'arguments : attend {} args, en reçoit {}", nom, args.len(), vars.len())
                }
            }
        },
        //None => panic!("{} n'est pas une fonction", nom),
        None => if vars.len() == 1 {
            let x = Var::Var(nom);
            eval_pap_fncall(x, vars[0].to_owned(), h, lfn)
        } else {
            todo!()
        }
    }
}

pub fn eval_cons_full_fn(args_fun:Vec<Var>, body_fun:FnBody, args:Vec<Var>, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    let mut heap = h.clone();
    for i in 0..args_fun.len() {
        heap = heap.add(args_fun[i].to_owned(), h.get(args[i].to_owned()));
    }
    eval_fnbody(body_fun, &heap, lfn)
}

pub fn eval_pap_fncall(x: Var, y: Var, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    let x_e = h.get(x);
    match x_e {
        Expr::Pap(c, mut vars) => {
            vars.push(y);
            eval_fncall(c, vars, h, lfn)
        },
        _ => panic!("Pas un pap"),
    }
}

pub  fn eval_pap(ident: Const, vars: Vec<Var>, _: &Heap, _:&mut HashMap<String,Fn>) -> Expr {
    Expr::Pap(ident, vars)
}



pub fn eval_ctor(n: i32, vars: Vec<Var>, _: &Heap, _:&mut HashMap<String,Fn>) -> Expr {
    let len:i32 = match vars.len().try_into() {
        Ok(v) => v,
        Err(_) => panic!("couldn't fit in i32"),
    };
    assert!(get_nb_args_ctor(n) == len);
    return create_ctor(n, vars);
}

pub  fn eval_proj(n: i32, var: Var, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    if let Expr::Ctor(_, exprs) = eval_var(var, h, lfn){
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("couldn't fit in usize"),
        };
        return eval_var(exprs[i].to_owned(), h, lfn);
    } else {
        panic!("Proj sur autre qu'un constructeur")
    }
}

pub  fn eval_value(n: i32, _: &Heap, _:&mut HashMap<String,Fn>) -> Expr {
    Expr::Num(n)
}

/*
* Fnbody evaluation section
*/
pub  fn eval_fnbody(body: FnBody, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    match body {
        FnBody::Ret(var) => eval_ret(var, h, lfn),
        FnBody::Let(var, expr, fnbody) => eval_let(var, expr, *fnbody, h, lfn),
        FnBody::Case(var, bodys) => eval_case(var, bodys, h, lfn),
    }
}

pub  fn eval_ret(var: Var, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    return eval_var(var, h, lfn);
}

pub  fn eval_let(var: Var, expr: Expr, fnbody: FnBody, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    let value = eval_expr(expr, h, lfn);
    let new_heap = h.add(var, value);
    let res = eval_fnbody(fnbody, &new_heap, lfn);
    res
}

pub  fn eval_case(var: Var, bodys: Vec<FnBody>, h: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    if let Expr::Ctor(n, _) = eval_var(var, h, lfn){
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("cannot fit i32 into usize"),
        };
        assert!(i + 1 <= bodys.len());
        eval_fnbody(bodys[i].to_owned(), h, lfn)
    } else {
        panic!("Case sur autre qu'un constructeur")
    }
}

/*
 * Eval Const
 */

pub fn eval_const(_: Const, _: &Heap, _:&mut HashMap<String,Fn>) -> Expr {
    panic!("Const ??")
}


pub fn eval_program(c: Const, fun:Fn, _: &Heap, lfn:&mut HashMap<String,Fn>) -> Expr {
    let Const::Const(nom) = c;
    lfn.insert(nom, fun);
    make_false()
}
