#[path = "../ast.rs"]
pub(crate) mod ast;

use std::collections;
use std::collections::HashMap;

use ast::Var;
use ast::Expr;
use ast::FnBody;
use ast::Fn;
use ast::AST;
use ast::Const;
use ast::CONST_FALSE;
use ast::CONST_TRUE;
use ast::CONST_NIL;
use ast::CONST_LIST;


#[derive(Debug, Clone)]
pub(crate) enum Heap{
    Some {
        var:ast::Var,
        value:Expr,
        next:Box<Heap>,
    },
    None,
}

pub(crate) fn start_interpreter (funs : Vec<AST>) -> Expr {
    let mut liste_fun:HashMap<String, Fn> = HashMap::new();
    let heap = empty_heap();
    let mut res = funs.iter()
        .fold((Expr::Num(0), empty_heap()), |(e, h), f| eval_ast(f.clone(), h, &mut liste_fun));
    return res.0;
}


impl Heap {
    pub(crate) fn get(&self, v:ast::Var) -> Expr{
        let ast::Var::Var(nom1) = &v;
        match self {
            Heap::Some { var, value, next } => {  
                let ast::Var::Var(nom2) = var;
                if *nom1 == *nom2 {
                    return value.clone();
                }
                return next.get(v);
            },
            Heap::None =>panic!("{} pas dans le heap", nom1)
        }
    }

    pub(crate) fn add(&self, v:ast::Var, val:Expr) -> Heap{
        return Heap::Some{
            var: v, 
            value: val, 
            next: Box::new(self.clone()),
        };
    }
}

pub(crate)  fn empty_heap() -> Heap {
    Heap::None
}

pub(crate)  fn make_false() -> Expr{
    return Expr::Ctor(CONST_FALSE, todo!());
}

pub(crate)  fn make_true() -> Expr{
    return Expr::Ctor(CONST_TRUE, todo!());
}

pub(crate)  fn throw() {
    panic!("Evaluation non défini pour ce type");
}

pub(crate)  fn get_nb_args_ctor(n: i32) -> i32 {
    match n {
        CONST_FALSE => 0,
        CONST_TRUE => 0,
        //CONST_NIL => 0,
        //CONST_LIST => 2,
        _ => panic!("Ctor {} non existant", n),
    }
}

pub(crate)  fn create_ctor(n: i32, args: Vec<Expr>) -> Expr {

    match n {
        CONST_FALSE => make_false(),
        CONST_TRUE => make_true(),
        //CONST_NIL => ,
        //CONST_LIST => ,
        _ => panic!("Contructeur non connu")
    }
}

// Ast evaluation
pub(crate)  fn eval_ast(ast: AST, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
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
pub(crate) fn eval_var(var: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (h.get(var), h)
}

/*
* Expr evaluation section
*/

pub(crate)  fn eval_expr(expr: Expr, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    match expr {
        Expr::FnCall(ident, vars) => eval_fncall(ident, vars, h, lfn),
        Expr::PartialFnCall(x, y) => eval_pap_fncall(x, y, h, lfn),
        Expr::Pap(ident, vars) => eval_pap(ident, vars, h, lfn),
        Expr::Ctor(n, vars) => eval_ctor(n, vars, h, lfn),
        Expr::Proj(n, var) => eval_proj(n, var, h, lfn),
        Expr::Num(n) => eval_value(n, h, lfn),
    }
}

pub(crate)  fn eval_fun(fun:Fn, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (make_false(), h) 
}

pub(crate) fn eval_fncall(ident: Const, vars: Vec<Var>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let Const::Ident(nom) = &ident;
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

pub(crate) fn eval_cons_full_fn(args_fun:Vec<Var>, body_fun:FnBody, args:Vec<Var>, mut h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    for i in 0..args_fun.len() {
        h = h.add(args_fun[i].clone(), h.get(args[i].clone()))
    }
    let (r,_) = eval_fnbody(body_fun, h.clone(), lfn);
    (r, h)
}

pub(crate) fn eval_pap_fncall(x: Var, y: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
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

pub(crate)  fn eval_pap(ident: Const, vars: Vec<Var>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (Expr::Pap(ident, vars), h)
}



pub(crate) fn eval_ctor(n: i32, vars: Vec<Var>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
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

pub(crate)  fn eval_proj(n: i32, var: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
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

pub(crate)  fn eval_value(n: i32, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    (Expr::Num(n), h)
}

/*
* Fnbody evaluation section
*/
pub(crate)  fn eval_fnbody(body: FnBody, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    match body {
        FnBody::Ret(var) => eval_ret(var, h, lfn),
        FnBody::Let(var, expr, fnbody) => eval_let(var, expr, *fnbody, h, lfn),
        FnBody::Case(var, bodys) => eval_case(var, *bodys, h, lfn),
    }
}

pub(crate)  fn eval_ret(var: Var, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    return eval_var(var, h, lfn);
}

pub(crate)  fn eval_let(var: Var, expr: Expr, fnbody: FnBody, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let (value, h) = eval_expr(expr, h, lfn);
    let new_heap = h.add(var, value);
    let (res, _) = eval_fnbody(fnbody, new_heap, lfn);
    return (res, h);
}

pub(crate)  fn eval_case(var: Var, bodys: Vec<FnBody>, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
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

pub(crate) fn eval_const(c: Const, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    panic!("Const ??")
}


pub(crate) fn eval_program(c: Const, fun:Fn, h: Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let Const::Ident(nom) = c;
    unsafe {
        lfn.insert(nom, fun)
    };
    (make_false(), h)
}