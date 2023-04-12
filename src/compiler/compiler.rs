#![allow(dead_code)]

#[path = "../ast.rs"]
pub mod ast;

use std::fs::File;
use crate::ast::Program;
pub use crate::ast::Var;
pub use crate::ast::Expr;
pub use crate::ast::FnBody;
pub use crate::ast::Fn;
pub use crate::ast::AST;
pub use crate::ast::Const;
pub use crate::ast::CONST_FALSE;
pub use crate::ast::CONST_TRUE;
pub use crate::ast::CONST_NIL;
pub use crate::ast::CONST_LIST;


pub mod primitives;
use primitives::is_primitive;
use primitives::compile_fncall_primitive;

use self::primitives::write_ln;
use self::primitives::write_out;



pub fn make_false(out:&mut File) {

}

pub fn make_true(out:&mut File) {

}

pub fn make_nil(out:&mut File) {

}

pub  fn make_list(out:&mut File) {

}

pub  fn make_num(out:&mut File) {

}


pub fn compile(parsed : Vec<AST>, out : &mut File){

    write_ln("(module", out);
    write_ln("(memory (import \"js\" \"mem\") 1)", out);
    for func in parsed {
        compile_ast(func, out);
    }
    write_ln(")", out);

}


// Ast evaluation
pub  fn compile_ast(ast: AST, out : &mut File) {
    match ast {
        AST::Fn(fun) => compile_fun(fun, out),
        AST::Var(_) => panic!("Impossible de compiler une variable seule"),
        AST::Expr(expr) => compile_expr(expr, out),
        AST::FnBody(body) => compile_fnbody(body, out),
        AST::Const(_) => panic!("Impossible de compiler un Const seul"),
        AST::Program(prog) => compile_program(prog, out),
    }
}

/*
* Var evaluation section
*/
pub fn compile_var(var: Var, out : &mut File) {
    let v = string_of_var(var);
    write_ln(&format!("local.get ${v}"), out);
}

/*
* Expr evaluation section
*/

pub fn compile_expr(expr: Expr, out : &mut File) {
    match expr {
        Expr::FnCall(ident, vars) => compile_fncall(ident, vars, out),
        Expr::Pap(_, _) => todo!(),
        Expr::Ctor(n, vars) => compile_ctor(n, vars, out),
        Expr::Proj(n, var) => compile_proj(n, var, out),
        Expr::Num(n) => compile_value(n, out),
        Expr::PapCall(_, _) => todo!(),
        Expr::Reset(var) => compile_reset(var, out),
        Expr::Reuse(var, i, args) => compile_reuse(var, i, args, out),
    }
}

pub  fn compile_fun(fun:Fn, out : &mut File)  {

}


pub fn compile_fncall(ident: Const, vars:Vec<Var>, out : &mut File)  {

}



pub fn compile_ctor(n: i32, vars:Vec<Var>, out : &mut File)  {

}

// On commence à 1
pub fn compile_proj(n: i32, var:Var, out : &mut File)  {
    
}

pub  fn compile_value(n: i32, out : &mut File)  {
    write_ln(&format!("i32.const {n}"), out);
    make_num(out);
}

/*
* Fnbody evaluation section
*/
pub  fn compile_fnbody(body: FnBody, out : &mut File)  {
    match body {
        FnBody::Ret(var) => compile_ret(var, out),
        FnBody::Let(var, expr, fnbody) => compile_let(var, expr, *fnbody, out),
        FnBody::Case(var, bodys) => compile_case(var, bodys, out),
        FnBody::Inc(var, fnbody) => compile_inc(var, *fnbody, out),
        FnBody::Dec(var, fnbody) => compile_dec(var, *fnbody, out),
    }
}

fn string_of_var(Var::Var(s):Var) -> String {
    return s;
}

fn string_of_const(Const::Const(c):Const) -> String {
    return c;
}

pub  fn compile_ret(var: Var, out : &mut File)  {
    compile_var(var, out);
    write_ln("return", out);
}

pub  fn compile_let(var: Var, expr: Expr, fnbody:FnBody, out : &mut File)  {
    let v = string_of_var(var);
    compile_expr(expr, out);
    write_ln(&format!("local.set ${v}"), out);
    compile_fnbody(fnbody, out);
}

pub  fn compile_case(var: Var, bodys: Vec<FnBody>, out : &mut File)  {

}


pub fn compile_program(prog: Program, out : &mut File)  {
    let Program::Program(cste, fun) = prog;
    let Const::Const(nom) = cste;
    write_ln(&format!("(func ${nom} "), out);
    compile_fn(fun, out);
    write_ln(&format!("export \"{nom}\" (func ${nom}))"), out);
}

pub fn compile_fn(fun:Fn, out:&mut File){
    let Fn::Fn(params, fnbody) = fun;
    for param in params {
        let s = string_of_var(param);
        write_out(&format!("(param ${s} i32 ) "), out);
    }
    write_ln("(result i32)", out);
    compile_fnbody(fnbody, out);
}

pub fn init_var(var: Var, out: &mut File) {
    let s = string_of_var(var);
    write_ln(&format!("(local ${s} i32)"), out);
}

pub fn compile_inc(var: Var, fnbody:FnBody, out : &mut File)  {
    
    compile_fnbody(fnbody, out);
}

pub fn compile_dec(var: Var, fnbody:FnBody, out : &mut File)  {
    
    compile_fnbody(fnbody, out);
}


pub fn compile_reset(var: Var, out : &mut File)  {

}

pub fn compile_reuse(var: Var, ctor: i32, args: Vec<Var>, out: &mut File){

}