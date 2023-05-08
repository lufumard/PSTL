#![allow(dead_code)]

use std::collections::HashMap;

use indexmap::IndexMap;


use crate::ast::Program;
use crate::ast::Var;
use crate::ast::Expr;
use crate::ast::FnBody;
use crate::ast::Fn;
use crate::ast::Const;
use crate::compiler::reader_rc::var;
use crate::interpreter::primitives::has_args;
use crate::interpreter::primitives::is_primitive;


/*
* Util
*/

fn string_of_var(Var::Var(s):Var) -> String {
    return s;
}

fn add_var (ctxt_vars : &Vec<String>, name : String) -> Vec<String> {
    let mut new_ctxt_vars = ctxt_vars.clone();
    new_ctxt_vars.push(name);
    return new_ctxt_vars;
}

/*
* Program transformation
*/

pub fn transform_program (prog : Program) -> Program {
    let Program::Program(fun_dec) = prog;
    let new_funs : IndexMap<Const, Fn> = fun_dec.clone().iter()
        .map(|(cst, fun)| (cst.to_owned(), transform_fun(fun.to_owned(), vec![], &fun_dec)))
        .collect();
    Program::Program(new_funs)
}

/*
* Constant transformation
* Makes a variable if it needs to be one
*/

fn transform_const(cst : &Const, ctxt_vars : Vec<String>) -> Option<Var> {
    let Const::Const(nom) = cst;
    if ctxt_vars.contains(nom) {
        Some(Var::Var(nom.to_string()))
    } else {
        None
    }
}

/*
* Expr transformation section
*/

fn transform_expr(expr: Expr, ctxt_vars:Vec<String>, lfn:&IndexMap<Const, Fn>) -> Expr {
    match expr {
        Expr::FnCall(ident, vars) => transform_fncall(ident, vars, ctxt_vars, lfn),
        _ => expr
    }
}

fn transform_fncall(ident: Const, vars: Vec<Var>, ctxt_vars:Vec<String>, lfn:&IndexMap<Const, Fn>) -> Expr {
    match transform_const(&ident, ctxt_vars) {
        Some(var) => {
            if vars.len() != 1 {
                let str = string_of_var(var);
                panic!("{str} has the wrong amount of arguments (papfncall takes exactly 1 argument)");
            }
            Expr::PapCall(var, vars[0].to_owned())
        },
        None => {
            let Const::Const(name) = ident.clone();
            let diff = has_args(&name, vars.len() as i32);
            if is_primitive(&name){
                if diff == 0 {
                    return Expr::FnCall(ident, vars);
                } else if diff < 0 {
                    return Expr::Pap(ident, vars);
                } else {
                    panic!("Trop d'arguments pour la fonction")
                }
            } else {
                match lfn.get(&ident) {
                    Some(fun) => {
                        let Fn::Fn(params, _);
                        if params.len() == vars.len() {
                            return Expr::FnCall(ident, vars);
                        } else if params.len() > vars.len() {
                            return Expr::Pap(ident, vars);
                        } else {
                            panic!("Trop d'arguments pour la fonction")
                        }
                    },
                    None => panic!("La fonction n'existe pas"),
                }
            } 
            
        },
    }
}


/*
* Function transformation section
*/

fn transform_fun(fun:Fn, ctxt_vars:Vec<String>, lfn:&IndexMap<Const, Fn>) -> Fn {
    let Fn::Fn(vars, body) = fun;
    let new_ctxt_vars = vars.iter()
        .fold(ctxt_vars, |acc, v| add_var(&acc, string_of_var(v.to_owned())));
    Fn::Fn(vars, transform_fnbody(body, new_ctxt_vars, lfn))
}

/*
* Fnbody transformation section
*/
fn transform_fnbody(body: FnBody, ctxt_vars:Vec<String>, lfn:&IndexMap<Const, Fn>) -> FnBody {
    match body {
        FnBody::Let(var, expr, fnbody) => transform_let(var, expr, *fnbody, ctxt_vars, lfn),
        FnBody::Case(var, bodys) => transform_case(var, bodys, ctxt_vars, lfn),
        _ => body
    }
}

fn transform_let(var: Var, expr: Expr, fnbody: FnBody, ctxt_vars:Vec<String>, lfn:&IndexMap<Const, Fn>) -> FnBody {
    let new_ctxt_vars = add_var(&ctxt_vars, string_of_var(var.clone()));
    let new_expr = transform_expr(expr, ctxt_vars, lfn);
    let new_body = transform_fnbody(fnbody, new_ctxt_vars, lfn);
    return FnBody::Let(var, new_expr, Box::new(new_body));
}

fn transform_case(var: Var, bodys: Vec<FnBody>, ctxt_vars:Vec<String>, lfn:&IndexMap<Const, Fn>) -> FnBody {
    let new_bodys : Vec<FnBody> = bodys.iter()
        .map(|b| transform_fnbody(b.to_owned(), ctxt_vars.clone(), lfn))
        .collect();
    return FnBody::Case(var, new_bodys);
}
