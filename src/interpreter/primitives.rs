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



pub fn eval_fncall_primitive(nom: String, vars:Vec<Var>, h:Heap, lfn:&mut HashMap<String,Fn>) -> Expr{
    if (nom.clone().eq("not") && vars.len() != 1)||(!nom.clone().eq("not") && vars.len() != 2) {
        if (nom.clone().eq("not") && vars.len() > 1)||(!nom.clone().eq("not") && vars.len() > 2){
            panic!("Pas le bon nombre d'arguments pour la primitive {}, reçois {}", nom, vars.len() );
        };

        return Expr::Pap(Const::Const(nom), vars);
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
        "not" => not_fn(vars[0].to_owned(), h, lfn),
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

fn extract_int(e: Expr) -> i32 {
    match e {
        Num(n) => n,
        _ => panic!("Opération non défini pour ce type"),
    }
}

fn extract_bool(e: Expr) -> bool {
    match e {
        Ctor(n, _) => n == CONST_TRUE,
        _ => panic!("Opération non défini pour ce type"),
    }
}

// Définition des primitives
pub fn add_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    return Num(n + m);
}

pub fn sub_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    return Num(n - m);
}

pub fn mul_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    return Num(n * m);
}

pub fn div_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    return Num(n / m);
}

pub fn mod_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    return Num(n % m);
}

pub fn eq_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    
    if n==m { make_true() }    
    else { make_false() }
}

pub fn and_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let b = extract_bool(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    match b {
        false => make_false(),
        true => {
            let e = eval_expr(vars[1].to_owned(), h.clone(), lfn);
            if is_bool(&e) {
                e
            } else {
                panic!("Opération non définie entre bool et autre");
            }
        }
    }
}

pub fn or_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let b = extract_bool(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    match b {
        true => make_true(),
        false => {
            let e = eval_expr(vars[1].to_owned(), h.clone(), lfn);
            if is_bool(&e) {
                e
            } else {
                panic!("Opération non définie entre bool et autre");
            }
        }
    }
}

pub fn not_fn(var: Expr, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    let b = extract_bool(eval_expr(var, h.clone(), lfn));
    match b {
        false => make_true(),
        true => make_false(),
    }
}

pub fn sup_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    assert_eq!(vars.len(), 2);
    let n = extract_int(eval_expr(vars[0].to_owned(), h.clone(), lfn));
    let m = extract_int(eval_expr(vars[1].to_owned(), h.clone(), lfn));
    match n > m {
        true => make_true(),
        false => make_false(),
    }
}

pub fn inf_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    return sup_fn(vars.into_iter().rev().collect(), h, lfn);
}

pub fn sup_eq_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    return not_fn(inf_fn(vars, h.clone(), lfn), h, lfn);
}

pub fn inf_eq_fn(vars: Vec<Expr>, h: Heap, lfn: &mut HashMap<String, Fn>) -> Expr {
    sup_eq_fn(vars.into_iter().rev().collect(), h, lfn)
}
