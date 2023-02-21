//#[path = "../ast.rs"]
//mod ast;
//use ast::Expr;
//use ast::Expr::Ctor;
//use ast::Expr::Num;
/* */

//#[path = "./interpreter.rs"]
use std::collections::HashMap;

use crate::ast::Expr::Num;
use crate::ast::Expr::Ctor;
use crate::ast::Expr;
use crate::ast::Var;
use crate::ast::Const;
use crate::ast::Fn;
use crate::interpreter::Heap;
use crate::interpreter::eval_expr;
use crate::interpreter::make_true;
use crate::interpreter::make_false;


use crate::ast::CONST_FALSE;
use crate::ast::CONST_TRUE;

#[allow(dead_code)]
const PRIMITIVES: [&str; 13]  = [
        "add", "sub", "mul", "div", "mod",
        "and", "or", "not",
        "eq", "sup", "inf", "sup_eq", "inf_eq"
    ];

pub fn is_primitive(nom: &String) -> bool {
    PRIMITIVES.contains(&nom.as_str())
}



pub fn eval_fncall_primitive(nom: String, vars:Vec<Var>, h:Heap, lfn:&mut HashMap<String,Fn>) -> (Expr, Heap){
    if (nom.clone().eq("not") && vars.len() != 1)||(!nom.clone().eq("not") && vars.len() != 2) {
        if (nom.clone().eq("not") && vars.len() > 1)||(!nom.clone().eq("not") && vars.len() > 2){
            panic!("Pas le bon nombre d'arguments pour la primitive {}, reçois {}", nom, vars.len() );
        };

        return (Expr::Pap(Const::Const(nom), vars), h);
    };

    let vars = vars.iter().map(|v| h.get(v.to_owned())).collect();
    
    match nom.as_str() {
        "add" => add_fn(vars, h, lfn),
        "sub" => sub_fn(vars, h, lfn),
        "mul" => mul_fn(vars, h, lfn),
        "div" => div_fn(vars, h, lfn),
        "mod" => mod_fn(vars, h, lfn),
        "and" => and_fn(vars, h, lfn),
        "or"  => or_fn (vars, h, lfn),
        "not" => not_fn(vars[0].clone(), h, lfn),
        "eq"  => eq_fn (vars, h, lfn),
        "sup" => sup_fn(vars, h, lfn),
        "inf" => inf_fn(vars, h, lfn),
        "sup_eq" => sup_eq_fn(vars, h, lfn),
        "inf_eq" => inf_eq_fn(vars, h, lfn),
        _ => panic!("La primitive {} n'existe pas", nom)
    }
}

fn is_bool(expr : &Expr) -> bool{
    match *expr {
        Ctor(CONST_FALSE, _) => true,
        Ctor(CONST_TRUE, _) => true,
        _ => false,
    }
}

fn extract_int((e, h): (Expr, Heap)) -> (i32, Heap) {
    match e {
        Num(n) => (n, h),
        _ => panic!("Opération non défini pour ce type"),
    }
}

fn extract_bool((e, h): (Expr, Heap)) -> (bool, Heap) {
    match e {
        Ctor(n, _) => (n == CONST_TRUE, h),
        _ => panic!("Opération non défini pour ce type"),
    }
}

// Définition des primitives
pub fn add_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n + m), h);
}

pub fn sub_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n - m), h);
}

pub fn mul_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n * m), h);
}

pub fn div_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n / m), h);
}

pub fn mod_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    return (Num(n % m), h);
}

pub fn eq_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    
    if n==m { (make_true(),h) }    
    else { (make_false(), h) }
}

pub fn and_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (b, _) = extract_bool(eval_expr(vars[0].clone(), h.clone(), lfn));
    match b {
        false => (make_false(), h),
        true => {
            let (e, _) = eval_expr(vars[1].clone(), h.clone(), lfn);
            if is_bool(&e) {
                (e, h)
            } else {
                panic!("Opération non définie entre bool et autre");
            }
        }
    }
}

pub fn or_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (b, _) = extract_bool(eval_expr(vars[0].clone(), h.clone(), lfn));
    match b {
        true => (make_true(), h),
        false => {
            let (e, _) = eval_expr(vars[1].clone(), h.clone(), lfn);
            if is_bool(&e) {
                (e, h)
            } else {
                panic!("Opération non définie entre bool et autre");
            }
        }
    }
}

pub fn not_fn(var: Expr, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    let (b, _) = extract_bool(eval_expr(var, h.clone(), lfn));
    match b {
        false => (make_true(), h),
        true => (make_false(), h),
    }
}

pub fn sup_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    assert_eq!(vars.len(), 2);
    let (n, _) = extract_int(eval_expr(vars[0].clone(), h.clone(), lfn));
    let (m, _) = extract_int(eval_expr(vars[1].clone(), h.clone(), lfn));
    match n > m {
        true => (make_true(), h),
        false => (make_false(), h),
    }
}

pub fn inf_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    return sup_fn(vars.into_iter().rev().collect(), h, lfn);
}

pub fn sup_eq_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    let (e, h) = inf_fn(vars, h, lfn);
    return not_fn(e, h, lfn);
}

pub fn inf_eq_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> (Expr, Heap) {
    sup_eq_fn(vars.into_iter().rev().collect(), h, lfn)
}
