//#[path = "../ast.rs"]
//mod ast;
//use ast::Expr;
//use ast::Expr::Ctor;
//use ast::Expr::Num;
/* */

//#[path = "./interpreter.rs"]
use std::collections::HashMap;

use crate::ast::Const;
use crate::ast::Fn;
use crate::interpreter::Ctxt;
use crate::interpreter::Loc;
use crate::interpreter::Heap;
use crate::interpreter::Value::Ctor;
use crate::interpreter::Value::Num;
use crate::interpreter::Value::Pap;
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



pub fn eval_fncall_primitive(nom: String, vars:Vec<Loc>, c: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String,Fn>) -> Loc{
    if (nom.clone().eq("not") && vars.len() != 1)||(!nom.clone().eq("not") && vars.len() != 2) {
        if (nom.clone().eq("not") && vars.len() > 1)||(!nom.clone().eq("not") && vars.len() > 2){
            panic!("Pas le bon nombre d'arguments pour la primitive {}, reçois {}", nom, vars.len() );
        };
        let v = Pap(Const::Const(nom), vars);
        return h.add((v, 1));
    };
    
    match nom.as_str() {
        "add" => add_fn(vars, c, h, lfn),
        "sub" => sub_fn(vars, c, h, lfn),
        "mul" => mul_fn(vars, c, h, lfn),
        "div" => div_fn(vars, c, h, lfn),
        "mod" => mod_fn(vars, c, h, lfn),
        "and" => and_fn(vars, c, h, lfn),
        "or"  => or_fn (vars, c, h, lfn),
        "not" => not_fn(vars[0].to_owned(), c, h, lfn),
        "eq"  => eq_fn (vars, c, h, lfn),
        "sup" => sup_fn(vars, c, h, lfn),
        "inf" => inf_fn(vars, c, h, lfn),
        "sup_eq" => sup_eq_fn(vars, c, h, lfn),
        "inf_eq" => inf_eq_fn(vars, c, h, lfn),
        _ => panic!("La primitive {} n'existe pas", nom)
    }
}

fn is_bool(l : &Loc, h:&mut Heap) -> bool{
    match h.get(l.clone()) {
        (Ctor(CONST_FALSE, _), _) => true,
        (Ctor(CONST_TRUE, _), _) => true,
        _ => false,
    }
}

fn extract_int(l: Loc, h:&mut Heap) -> i32 {
    match h.get(l) {
        (Num(n), _) => n,
        _ => panic!("Opération non défini pour ce type"),
    }
}

fn extract_bool(l: Loc, h:&mut Heap) -> bool {
    match h.get(l) {
        (Ctor(CONST_FALSE, _), _) => false,
        (Ctor(CONST_TRUE, _), _) => true,
        _ => panic!("Opération non défini pour ce type"),
    }
}


// Définition des primitives
pub fn add_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    return heap.add((Num(n + m), 1));
}

pub fn sub_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    return heap.add((Num(n - m), 1));
}

pub fn mul_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    return heap.add((Num(n * m), 1));
}

pub fn div_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    return heap.add((Num(n / m), 1));
}

pub fn mod_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    return heap.add((Num(n % m), 1));
}

pub fn eq_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    let v = if n==m { make_true() } else { make_false() };
    return heap.add((v, 1));
}

pub fn and_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let b = extract_bool(vars[0].to_owned(), heap);
    if !b {
        return heap.add((make_false(), 1));
    }
    if is_bool(&vars[1], heap) {
        return vars[1];
    }
    panic!("Opération and non définie entre bool et autre");
}

pub fn or_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let b = extract_bool(vars[0].to_owned(), heap);
    if !b {
        return heap.add((make_true(), 1));
    }
    if is_bool(&vars[1], heap) {
        return vars[1];
    }
    panic!("Opération or non définie entre bool et autre");
}

pub fn not_fn(var: Loc, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    let b = extract_bool(var, heap);
    match b {
        false => heap.add((make_true(), 1)),
        true => heap.add((make_false(), 1)),
    }
}

pub fn sup_fn(vars: Vec<Loc>, _: &Ctxt, heap:&mut Heap, _: &mut HashMap<String, Fn>) -> Loc {
    assert_eq!(vars.len(), 2);
    let n = extract_int(vars[0].to_owned(), heap);
    let m = extract_int(vars[1].to_owned(), heap);
    match n > m {
        true => heap.add((make_true(), 1)),
        false => heap.add((make_false(), 1)),
    }
}

pub fn inf_fn(vars: Vec<Loc>, ct: &Ctxt, heap:&mut Heap, lfn: &mut HashMap<String, Fn>) -> Loc {
    return sup_fn(vars.into_iter().rev().collect(), ct, heap, lfn);
}

pub fn sup_eq_fn(vars: Vec<Loc>, ct: &Ctxt, heap:&mut Heap, lfn: &mut HashMap<String, Fn>) -> Loc {
    return not_fn(inf_fn(vars, ct, heap, lfn), ct, heap, lfn);
}

pub fn inf_eq_fn(vars: Vec<Loc>, ct: &Ctxt, heap:&mut Heap, lfn: &mut HashMap<String, Fn>) -> Loc {
    return sup_eq_fn(vars.into_iter().rev().collect(), ct, heap, lfn);
}
