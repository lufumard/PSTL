use std::collections::HashMap;
use std::fmt::write;
use std::fs::File;

use crate::ast::Var;
use crate::compiler::compile_var;
use crate::compiler::make_num;
use crate::compiler::make_true;
use crate::compiler::make_false;


use super::string_of_var;

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

pub fn compile_fncall_primitive(nom: String, vars:Vec<Var>, out:&mut File) {  
    match nom.as_str() {
        "add" => add_fn(vars, out),
        "sub" => sub_fn(vars, out),
        "mul" => mul_fn(vars, out),
        "div" => div_fn(vars, out),
        "mod" => mod_fn(vars, out),
        "and" => and_fn(vars, out),
        "or"  => or_fn (vars, out),
        "not" => not_fn(vars[0].to_owned(), out),
        "eq"  => eq_fn (vars, out),
        "sup" => sup_fn(vars, out),
        "inf" => inf_fn(vars, out),
        "sup_eq" => sup_eq_fn(vars, out),
        "inf_eq" => inf_eq_fn(vars, out),
        _ => panic!("La primitive {} n'existe pas", nom)
    }
}

pub fn write_out(s : &str, out : &mut File){
    todo!()
}

pub fn write_ln(s : &str, out : &mut File){
    write_out(&format!("{s}\n"), out);
}


pub fn get_num(var:Var, out : &mut File) {
    let s = string_of_var(var);
    write_ln(&format!("(i32.load (i32.add (local.get ${s}) (8)))"), out);
}

pub fn get_bool(var:Var, out : &mut File) {
    compile_var(var, out);
}

// Définition des primitives
// TODO : rendre plus concis

pub fn add_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(i32.add (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln("))", out);
    make_num(out);
}

pub fn sub_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(i32.sub (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln("))", out);
    make_num(out);
}

pub fn mul_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(i32.mul (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln("))", out);
    make_num(out);
}

pub fn div_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(i32.div_s (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln("))", out);
    make_num(out);
}

pub fn mod_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.len(), 2);
    write_ln("(i32.rem_s (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln("))", out);
    make_num(out);
}

pub fn eq_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.eq (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (", out);
    make_false(out);
    write_ln(")))", out);
}

pub fn and_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.eq (1) (", out);
    get_bool(vars[0].to_owned(), out);
    write_ln(")) (then (if (i32.eq (1) (", out);
    get_bool(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")))) (else (", out);
    make_false(out);
    write_ln(")))", out);
}


pub fn or_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.eq (1) (", out);
    get_bool(vars[0].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (if (i32.eq (1) (", out);
    get_bool(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (", out);
    make_false(out);
    write_ln(")))))", out);
}

pub fn not_fn(var : Var, out:&mut File) {
    write_ln("(if (i32.eq (1) (", out);
    get_bool(var, out);
    write_ln(")) (then", out);
    make_true(out);
    write_ln(")) (else (", out);
    make_false(out);
    write_ln(")))", out);
}

pub fn sup_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.gt_s (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (", out);
    write_ln(")))", out);
}

pub fn inf_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.lt_s (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (", out);
    make_false(out);
    write_ln(")))", out);
}

pub fn sup_eq_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.ge_s (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (", out);
    make_false(out);
    write_ln(")))", out);
}

pub fn inf_eq_fn(vars: Vec<Var>, out:&mut File) {
    assert_eq!(vars.len(), 2);
    write_ln("(if (i32.le_s (", out);
    get_num(vars[0].to_owned(), out);
    write_ln(") (", out);
    get_num(vars[1].to_owned(), out);
    write_ln(")) (then (", out);
    make_true(out);
    write_ln(")) (else (", out);
    make_false(out);
    write_ln(")))", out);
}
