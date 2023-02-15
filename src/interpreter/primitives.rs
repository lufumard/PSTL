//#[path = "../ast.rs"]
//mod ast;
//use ast::Expr;
//use ast::Expr::Ctor;
//use ast::Expr::Num;

#[path = "interpreter.rs"]
mod interpreter;
use std::collections::HashMap;

use interpreter::ast::Expr::Num;
use interpreter::ast::Expr::Ctor;
use interpreter::ast::Expr;
use interpreter::ast::Fn;
use interpreter::Heap;
use interpreter::eval_expr;
use interpreter::make_true;
use interpreter::make_false;

use crate::ast::CONST_FALSE;
use crate::ast::CONST_TRUE;




fn is_bool(expr : &Expr) -> bool{
    match *expr {
        Ctor(CONST_FALSE, _) => true,
        Ctor(CONST_TRUE, _) => true,
        _ => false
    }
}

fn extract_int((e, h):(Expr, Heap)) -> (i32, Heap) {
    match e {
        Num(n) => (n, h),
        _ => panic!("Opération non défini pour ce type")
    }
}

fn extract_bool((e, h):(Expr, Heap)) -> (bool, Heap) {
    match e {
        Ctor(n, _) => (n == CONST_TRUE, h), 
        _ => panic!("Opération non défini pour ce type")
    }
}

// Définition des primitives
pub(crate)  fn add_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n+m), h);
}

pub(crate)  fn sub_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n-m), h);
}

pub(crate)  fn mul_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n*m), h);
}

pub(crate)  fn div_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n/m), h);
}

pub(crate) fn mod_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n%m), h);
}

pub(crate)  fn and_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (b, _) = extract_bool(eval_expr(vars[0].clone(), h.clone(), lfn));
    match b {
        false => (make_false(), h),
        true => {
            let (e, _) = eval_expr(vars[1].clone(), h.clone(), lfn);
            if is_bool(&e) { (e, h) }
            else { panic!("Opération non définie entre bool et autre"); }
        }
    }
}

pub(crate)  fn or_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap){
    assert_eq!(vars.len(), 2);
    let (b, _) = extract_bool(eval_expr(vars[0].clone(), h.clone(), lfn));
    match b {
        true => (make_true(), h),
        false => {
            let (e, _) = eval_expr(vars[1].clone(), h.clone(), lfn);
            if is_bool(&e) { (e, h) }
            else { panic!("Opération non définie entre bool et autre"); }
        }
    }
}

pub(crate)  fn not_fn(var:Expr, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let (b, _) = extract_bool(eval_expr(var, h.clone(), lfn));
    match b {
        false => (make_true(), h),
        true => (make_false(), h),
    }
}

pub(crate)  fn sup_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    match n>m {
        true  => (make_true(), h),
        false => (make_false(), h)
    }
}

pub(crate)  fn inf_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    return sup_fn(vars.into_iter().rev().collect(), h, lfn);
}

pub(crate)  fn sup_eq_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    let (e, h) = inf_fn(vars, h, lfn);
    return not_fn(e, h, lfn);
}

pub(crate)  fn inf_eq_fn(vars:Vec<Expr>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap) {
    sup_eq_fn(vars.into_iter().rev().collect(), h, lfn)
}