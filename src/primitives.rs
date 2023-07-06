use std::fs::File;

use crate::{interpreter::{Loc, Heap, CONST_FALSE, CONST_TRUE, CONST_NUM}, compiler::{Var, write_ln}};
use crate::interpreter::Value::Ctor;

pub fn extract_int(l: Loc, h:&Heap) -> i32 {
    match h.get(l) {
        Ctor(CONST_NUM, ls) => {
            let Loc::Loc(n) = ls[0];
            n
        },
        _ => panic!("Opération non défini pour ce type"),
    }
}

pub fn extract_bool(l: Loc, h:&Heap) -> bool {
    match h.get(l) {
        Ctor(CONST_FALSE, _) => false,
        Ctor(CONST_TRUE, _) => true,
        _ => panic!("Opération non défini pour ce type"),
    }
}


// Compiler


#[derive(Clone)]
pub enum PrimitiveFunction {
    BooleanToBoolean (&'static dyn Fn(Vec<bool>) -> bool),
    BooleanToNumeric (&'static dyn Fn(Vec<bool>) -> i32),
    NumericToBoolean (&'static dyn Fn(Vec<i32>) -> bool),
    NumericToNumeric (&'static dyn Fn(Vec<i32>) -> i32),
}

pub trait PrimitiveTrait {
    fn apply_inter(self, args : Vec<Loc>, heap: &mut Heap) -> Loc;

    fn apply_compil(self, vars: &Vec<Var>, out:&mut File);
}

#[derive(Clone)]
pub struct Primitive {
    name: &'static str,
    nb_args : usize,
    fun_inter : PrimitiveFunction,
    fun_compil : &'static str
}

const fn new_primitive(name : &'static str, nb_args : usize, fun_inter : PrimitiveFunction, fun_compil : &'static str) -> Primitive {
    return Primitive {
        name : name,    
        nb_args : nb_args,
        fun_inter : fun_inter,
        fun_compil : fun_compil,
    }
}

impl PrimitiveTrait for Primitive {
    fn apply_inter(self, args : Vec<Loc>, heap : &mut Heap) -> Loc {
        assert_eq!(args.len(), self.nb_args.into());

        let value = match self.fun_inter {
            PrimitiveFunction::BooleanToBoolean(fun) =>
                crate::interpreter::make_bool((*fun)(args.iter().map(|&l| extract_bool(l, heap)).collect())),
            PrimitiveFunction::BooleanToNumeric(fun) => 
                crate::interpreter::make_num((*fun)(args.iter().map(|&l| extract_bool(l, heap)).collect())),
            PrimitiveFunction::NumericToBoolean(fun) => 
                crate::interpreter::make_bool((*fun)(args.iter().map(|&l| extract_int(l, heap)).collect())),
            PrimitiveFunction::NumericToNumeric(fun) => 
                crate::interpreter::make_num((*fun)(args.iter().map(|&l| extract_int(l, heap)).collect())),
        };

        return heap.add(value);
    }


    fn apply_compil(self, vars : &Vec<Var>, out:&mut File) {
        assert!(vars.len() >= self.nb_args);

        match self.fun_inter {
            PrimitiveFunction::BooleanToBoolean(_) | PrimitiveFunction::BooleanToNumeric(_)=> 
                vars.iter().for_each(|var| crate::compiler::get_bool(var, out)),
            PrimitiveFunction::NumericToBoolean(_) | PrimitiveFunction::NumericToNumeric(_) => 
                vars.iter().for_each(|var| crate::compiler::get_num(var, out)),
        }

        write_ln(self.fun_compil, out);


        match self.fun_inter {
            PrimitiveFunction::BooleanToBoolean(_) | PrimitiveFunction::NumericToBoolean(_)=> 
                crate::compiler::make_bool(out),
            PrimitiveFunction::BooleanToNumeric(_) | PrimitiveFunction::NumericToNumeric(_) => 
                crate::compiler::make_num(out)
        }

        vars.iter().for_each(|var| {
            crate::compiler::compile_var(var, out);
            write_ln("call $__dec", out);    
        });
    }
}




fn get_primitive(nom: &str) -> Option<Primitive> {
    get_primitives().iter().fold(None, |x, info| 
         if x.is_none() && info.name.eq(nom) {
            Some(info.clone())
        } else {
            x
        }
    )
}
    
pub fn is_primitive(nom: &str) -> bool {
    get_primitives().iter().any(|info| info.name.eq(nom))
}

pub fn nb_args(nom : &str) -> usize {
    if let Some(info) = get_primitive(nom) {
        return info.nb_args;
    } else {
        panic!("la primitive {nom} n'existe pas")
    }
}

fn get_primitives() -> Vec<Primitive> {
    return Vec::from(PRIMITIVES);
}

pub fn get_primitive_names() -> Vec<String> {
    let vec = get_primitives().iter().map(|info| info.name.to_string()).collect();
    return vec;
}

pub fn eval_fncall_primitive(nom: String, vars:Vec<Loc>, h:&mut Heap) -> Loc{  
    match get_primitive(&nom.as_str()) {
        Some(info) => info.apply_inter(vars, h),
        _ => panic!("La primitive {} n'existe pas", nom)
    }
}

pub fn compile_fncall_primitive(nom: &String, vars:&Vec<Var>, out:&mut File) {  
    match get_primitive(nom) {
        Some(info) => info.apply_compil(vars, out),
        _ => panic!("La primitive {} n'existe pas", nom)
    }
}

pub fn has_args(nom : &String, length:usize) -> i32 {
    if let Some(fun) = get_primitive(nom) {
        return length as i32 - fun.nb_args as i32
    }
    panic!("Unknown primitive {nom}")
}

fn add(args :Vec<i32>) -> i32 { args[0] + args[1] }
fn sub(args :Vec<i32>) -> i32 { args[0] - args[1] }
fn mul(args :Vec<i32>) -> i32 { args[0] * args[1] }
fn div(args :Vec<i32>) -> i32 { args[0] / args[1] }
fn _mod(args :Vec<i32>) -> i32 { args[0] % args[1] }

fn and(args: Vec<bool>) -> bool { args[0] && args[1] }
fn or(args: Vec<bool>) -> bool { args[0] || args[1] }
fn not(args: Vec<bool>) -> bool { !args[0] }

fn eq(args: Vec<i32>) -> bool { args[0] == args[1] }
fn sup(args: Vec<i32>) -> bool { args[0] > args[1] }
fn inf(args: Vec<i32>) -> bool { args[0] < args[1] }
fn sup_eq(args: Vec<i32>) -> bool { args[0] >= args[1] }
fn inf_eq(args: Vec<i32>) -> bool { args[0] <= args[1] }

const PRIMITIVES: [Primitive; 13] = [
    new_primitive("add", 2, PrimitiveFunction::NumericToNumeric(&add), "i32.add"),
    new_primitive("sub", 2, PrimitiveFunction::NumericToNumeric(&sub), "i32.sub"),
    new_primitive("mul", 2, PrimitiveFunction::NumericToNumeric(&mul), "i32.mul"),
    new_primitive("div", 2, PrimitiveFunction::NumericToNumeric(&div), "i32.div_s"),
    new_primitive("mod", 2, PrimitiveFunction::NumericToNumeric(&_mod), "i32.rem_s"),

    new_primitive("and", 2, PrimitiveFunction::BooleanToBoolean(&and), "i32.and"),
    new_primitive("or", 2, PrimitiveFunction::BooleanToBoolean(&or), "i32.or"),
    new_primitive("not", 1, PrimitiveFunction::BooleanToBoolean(&not), "i32.eqz"),

    new_primitive("eq", 2, PrimitiveFunction::NumericToBoolean(&eq), "i32.eq"),
    new_primitive("sup", 2, PrimitiveFunction::NumericToBoolean(&sup), "i32.gt_s"),
    new_primitive("inf", 2, PrimitiveFunction::NumericToBoolean(&inf), "i32.lt_s"),
    new_primitive("sup_eq", 2, PrimitiveFunction::NumericToBoolean(&sup_eq), "i32.ge_s"),
    new_primitive("inf_eq", 2, PrimitiveFunction::NumericToBoolean(&inf_eq), "i32.le_s"),
];

