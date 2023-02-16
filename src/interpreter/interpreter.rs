#[path = "../ast.rs"]
pub mod ast;

use std::collections;
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


/*

use crate::primitives::add_fn;
use crate::primitives::sub_fn;
use crate::primitives::mul_fn;
use crate::primitives::div_fn;
use crate::primitives::mod_fn;
use crate::primitives::and_fn;
use crate::primitives::or_fn;
use crate::primitives::not_fn;
use crate::primitives::inf_fn;
use crate::primitives::sup_fn;
use crate::primitives::inf_eq_fn;
use crate::primitives::sup_eq_fn;
*/

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

pub fn start_interpreter (funs : Vec<AST>) -> Expr {
    let mut liste_fun:HashMap<String, Fn> = HashMap::new();
    let heap = empty_heap();
    let mut res = funs.iter()
        .fold((Expr::Num(0), empty_heap()), |(e, h), f| eval_ast(f.clone(), h, &mut liste_fun));
    return res.0;
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
            },
            Heap::None =>panic!("{} pas dans le heap", nom1)
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
    return Expr::Ctor(CONST_FALSE, todo!());
}

pub  fn make_true() -> Expr{
    return Expr::Ctor(CONST_TRUE, todo!());
}

pub  fn throw() {
    panic!("Evaluation non défini pour ce type");
}

pub  fn get_nb_args_ctor(n: i32) -> i32 {
    match n {
        CONST_FALSE => 0,
        CONST_TRUE => 0,
        //CONST_NIL => 0,
        //CONST_LIST => 2,
        _ => panic!("Ctor {} non existant", n),
    }
}

pub  fn create_ctor(n: i32, args: Vec<Expr>) -> Expr {

    match n {
        CONST_FALSE => make_false(),
        CONST_TRUE => make_true(),
        //CONST_NIL => ,
        //CONST_LIST => ,
        _ => panic!("Contructeur non connu")
    }
}

// Ast evaluation
pub  fn eval_ast(ast: AST, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
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
pub fn eval_var(var: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (h.get(var), h)
}

/*
* Expr evaluation section
*/

pub  fn eval_expr(expr: Expr, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    match expr {
        Expr::FnCall(ident, vars) => eval_fncall(ident, vars, h, lfn),
        Expr::PartialFnCall(x, y) => eval_pap_fncall(x, y, h, lfn),
        Expr::Pap(ident, vars) => eval_pap(ident, vars, h, lfn),
        Expr::Ctor(n, vars) => eval_ctor(n, vars, h, lfn),
        Expr::Proj(n, var) => eval_proj(n, var, h, lfn),
        Expr::Num(n) => eval_value(n, h, lfn),
    }
}

pub  fn eval_fun(fun:Fn, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (make_false(), h) 
}


pub fn eval_fncall(ident: Const, vars: Vec<Var>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let Const::Ident(nom) = &ident;
    if is_primitive(nom) {
        let exprs = vars.iter().map(|v| h.get(v.to_owned())).collect();
        return eval_fncall_primitive(nom, exprs, h, lfn);
    }
    unsafe {
        match lfn.get(nom).cloned() {
            Some(Fn::Fn(args, body)) => {
                if args.len() == vars.len(){
                    eval_cons_full_fn(args, body, vars, h, lfn)
                }else {
                    if args.len() < vars.len(){
                        eval_pap(ident, vars, h, lfn)
                    }else {
                        panic!("{} n'a pas autant d'arguments", nom)
                    }
                }
            },
            None => panic!("{} n'est pas une fonction", nom) 
        }  
    }
}

pub fn eval_cons_full_fn(args_fun:Vec<Var>, body_fun:FnBody, args:Vec<Var>, mut h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    for i in 0..args_fun.len() {
        h = h.add(args_fun[i].clone(), h.get(args[i].clone()))
    }
    let (r,_) = eval_fnbody(body_fun, h.clone(), lfn);
    (r, h)
}

pub fn eval_pap_fncall(x: Var, y: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let x_e = h.get(x);
    match x_e {
        Expr::Pap(c, mut vars) => {
            vars.push(y);
            let (res, _) = eval_fncall(c, vars, h.clone(), lfn);
            (res, h)
        },
        _ => panic!("Pas un pap")        
    }
}

pub  fn eval_pap(ident: Const, vars: Vec<Var>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (Expr::Pap(ident, vars), h)
}



pub fn eval_ctor(n: i32, vars: Vec<Var>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let len:i32 = match vars.len().try_into() {
        Ok(v) => v,
        Err(_) => panic!("couldn't fit in i32")
    };
    assert!(get_nb_args_ctor(n) == len);
    let args = vars
            .into_iter()
            .map(|a| {let (v,_) = eval_var(a, h.clone(), lfn); v})
            .collect();
    return (create_ctor(n, args), h);
}

pub  fn eval_proj(n: i32, var: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    if let (Expr::Ctor(_, exprs), h) = eval_var(var, h, lfn){
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("couldn't fit in usize")
        };
        return eval_var(exprs[i].clone(), h, lfn);
    }else {
        panic!("Proj sur autre qu'un constructeur")
    }
}

pub  fn eval_value(n: i32, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (Expr::Num(n), h)
}

/*
* Fnbody evaluation section
*/
pub  fn eval_fnbody(body: FnBody, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    match body {
        FnBody::Ret(var) => eval_ret(var, h, lfn),
        FnBody::Let(var, expr, fnbody) => eval_let(var, expr, *fnbody, h, lfn),
        FnBody::Case(var, bodys) => eval_case(var, *bodys, h, lfn),
    }
}

pub  fn eval_ret(var: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    return eval_var(var, h, lfn);
}

pub  fn eval_let(var: Var, expr: Expr, fnbody: FnBody, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let (value, h) = eval_expr(expr, h, lfn);
    let new_heap = h.add(var, value);
    let (res, _) = eval_fnbody(fnbody, new_heap, lfn);
    return (res, h);
}

pub  fn eval_case(var: Var, bodys: Vec<FnBody>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    if let (Expr::Ctor(n, _), h) = eval_var(var, h, lfn){
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("cannot fit i32 into usize")
        };
        assert!(i+1<=bodys.len());
        eval_fnbody(bodys[i].clone(), h, lfn)
    } else {
        panic!("Case sur autre qu'un constructeur")
    }
    
}


/*
 * Eval Const
 */

pub fn eval_const(c: Const, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    panic!("Const ??")
}


pub fn eval_program(c: Const, fun:Fn, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let Const::Ident(nom) = c;
    unsafe {
        lfn.insert(nom, fun)
    };
    (make_false(), h)
}