#![allow(dead_code)]

use indexmap::IndexMap;


use crate::ast::Program;
use crate::ast::Var;
use crate::ast::Expr;
use crate::ast::FnBody;
use crate::ast::Fn;
use crate::ast::Const;

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
    let new_funs : IndexMap<Const, Fn> = fun_dec.iter()
        .map(|(cst, fun)| (cst.to_owned(), transform_fun(fun.to_owned(), vec![])))
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

fn transform_expr(expr: Expr, ctxt_vars:Vec<String>) -> Expr {
    match expr {
        Expr::FnCall(ident, vars) => transform_fncall(ident, vars, ctxt_vars),
        _ => expr
    }
}

fn transform_fncall(ident: Const, vars: Vec<Var>, ctxt_vars:Vec<String>) -> Expr {
    match transform_const(&ident, ctxt_vars) {
        Some(var) => {
            if vars.len() != 1 {
                let str = string_of_var(var);
                panic!("{str} has the wrong amount of arguments (papfncall takes exactly 1 argument)");
            }
            Expr::PapCall(var, vars[0].to_owned())
        },
        None => Expr::FnCall(ident, vars),
    }
}


/*
* Function transformation section
*/

fn transform_fun(fun:Fn, ctxt_vars:Vec<String>) -> Fn {
    let Fn::Fn(vars, body) = fun;
    let new_ctxt_vars = vars.iter()
        .fold(ctxt_vars, |acc, v| add_var(&acc, string_of_var(v.to_owned())));
    Fn::Fn(vars, transform_fnbody(body, new_ctxt_vars))
}

/*
* Fnbody transformation section
*/
fn transform_fnbody(body: FnBody, ctxt_vars:Vec<String>) -> FnBody {
    match body {
        FnBody::Let(var, expr, fnbody) => transform_let(var, expr, *fnbody, ctxt_vars),
        FnBody::Case(var, bodys) => transform_case(var, bodys, ctxt_vars),
        _ => body
    }
}

fn transform_let(var: Var, expr: Expr, fnbody: FnBody, ctxt_vars:Vec<String>) -> FnBody {
    let new_ctxt_vars = add_var(&ctxt_vars, string_of_var(var.clone()));
    let new_expr = transform_expr(expr, ctxt_vars);
    let new_body = transform_fnbody(fnbody, new_ctxt_vars);
    return FnBody::Let(var, new_expr, Box::new(new_body));
}

fn transform_case(var: Var, bodys: Vec<FnBody>, ctxt_vars:Vec<String>) -> FnBody {
    let new_bodys : Vec<FnBody> = bodys.iter()
        .map(|b| transform_fnbody(b.to_owned(), ctxt_vars.clone()))
        .collect();
    return FnBody::Case(var, new_bodys);
}
