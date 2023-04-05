use std::collections::HashMap;
use std::fs::File;

use crate::ast::Fn;
use crate::ast::Var;
use crate::interpreter::make_num;
use crate::interpreter::make_true;
use crate::interpreter::make_false;


use crate::ast::CONST_FALSE;
use crate::ast::CONST_TRUE;
use crate::ast::CONST_NUM;
use crate::ast::CONST_LIST;
use crate::ast::CONST_NIL;

#[allow(dead_code)]
const PRIMITIVES: [&str; 13]  = [
        "add", "sub", "mul", "div", "mod",
        "and", "or", "not",
        "eq", "sup", "inf", "sup_eq", "inf_eq"
    ];

pub fn is_primitive(nom: &String) -> bool {
    PRIMITIVES.contains(&nom.as_str())
}

pub fn has_args(nom : &String, length:i32) -> i32 {
    if nom.clone().eq("not") {
        return length - 1;
    }
    return length - 2;
}

pub fn eval_fncall_primitive(nom: String, vars:Vec<Var>, lfn:&mut HashMap<String,Fn>) {  
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

pub fn write_ln(s : &str, out : &File){
    write(out, "{s}\n");
}




// Définition des primitives
// TODO : rendre plus concis

pub fn add_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(call (make_num (i32.add (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln("))))");
}

pub fn sub_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(call (make_num (i32.sub (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln("))))");
}

pub fn mul_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(call (make_num (i32.mul (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln("))))");
}

pub fn div_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(call (make_num (i32.div_s (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln("))))");
}

pub fn mod_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(call (make_num (i32.rem_s (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln("))))");
}

pub fn eq_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.eq (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln(")) (then (call make_true)) (else (call make_false)))");
}

pub fn and_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    write_ln("(if (");
    get_bool(vars[0]);
    write_ln(") (then (if (i32.eq (1) (");
    get_bool(vars[1]);
    write_ln(") (then (call make_true))) (else (call make_false))))");
}


pub fn or_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    write_ln("(if (");
    get_bool(vars[0]);
    write_ln(") (then (call make_true)) (else (if (i32.eq (1) (");
    get_bool(vars[1]);
    write_ln(") (then (call make_true)) (else (call make_false)))))");
}

pub fn not_fn(var : Var)  {
    write_ln("(if (");
    get_bool(vars[0]);
    write_ln(") (then (call make_true)) (else (if (i32.eq (1) (");
    get_bool(vars[1]);
    write_ln(") (then (call make_true)) (else (call make_false)))))");
}

pub fn sup_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.gt_s (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln(")) (then (call make_true)) (else (call make_false)))");
}

pub fn inf_fn(vars: Vec<Var>) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.lt_s (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln(")) (then (call make_true)) (else (call make_false)))");
}

pub fn sup_eq_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.ge_s (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln(")) (then (call make_true)) (else (call make_false)))");
}

pub fn inf_eq_fn(vars: Vec<Var>)  {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.le_s (");
    get_num(vars[0]);
    write_ln(") (");
    get_num(vars[1]);
    write_ln(")) (then (call make_true)) (else (call make_false)))");
}
