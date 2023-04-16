 //#![allow(dead_code)]

pub mod ast_compiler;
pub mod ast_rc;
pub mod inferring;
pub mod reuse;
#[allow(non_snake_case)]
pub mod inc;
pub mod reader_compiler;
pub mod utils;

use std::fs::File;
use self::ast_compiler::Program;
pub use self::ast_compiler::Var;
pub use self::ast_compiler::Expr;
pub use self::ast_compiler::FnBody;
pub use self::ast_compiler::Fn;
pub use self::ast_compiler::AST;
pub use self::ast_compiler::Const;
pub use self::ast_compiler::CONST_FALSE;
pub use self::ast_compiler::CONST_TRUE;
pub use self::ast_compiler::CONST_NIL;
pub use self::ast_compiler::CONST_LIST;
use self::ast_rc::ExprRC;
use self::ast_rc::FnBodyRC;
use self::ast_rc::FnRC;
use self::ast_rc::ProgramRC;
use self::reuse::insert_reuse;


pub mod primitives;
#[allow(unused_imports)]
use primitives::is_primitive;
#[allow(unused_imports)]
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
    write_ln("i32.load 0", out);
}


pub fn compile(prog: Program, out : &mut File){

    write_ln("(module", out);
    write_ln("(memory (import \"js\" \"mem\") 1)", out);
    let prog_reuse = insert_reuse(prog);
    compile_program(prog_reuse, out);
    write_ln(")", out);

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

pub fn compile_expr(expr: ExprRC, out : &mut File) {
    match expr {
        ExprRC::FnCall(ident, vars) => compile_fncall(ident, vars, out),
        ExprRC::Pap(_, _) => todo!(),
        ExprRC::Ctor(n, vars) => compile_ctor(n, vars, out),
        ExprRC::Proj(n, var) => compile_proj(n, var, out),
        ExprRC::Num(n) => compile_value(n, out),
        ExprRC::PapCall(_, _) => todo!(),
        ExprRC::Reset(var) => compile_reset(var, out),
        ExprRC::Reuse(var, i, args) => compile_reuse(var, i, args, out),
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
pub  fn compile_fnbody(body: FnBodyRC, out : &mut File)  {
    match body {
        FnBodyRC::Ret(var) => compile_ret(var, out),
        FnBodyRC::Let(var, expr, fnbody) => compile_let(var, expr, *fnbody, out),
        FnBodyRC::Case(var, bodys) => compile_case(var, bodys, out),
        FnBodyRC::Inc(var, fnbody) => compile_inc(var, *fnbody, out),
        FnBodyRC::Dec(var, fnbody) => compile_dec(var, *fnbody, out),
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

pub  fn compile_let(var: Var, expr: ExprRC, fnbody:FnBodyRC, out : &mut File)  {
    let v = string_of_var(var);
    compile_expr(expr, out);
    write_ln(&format!("local.set ${v}"), out);
    compile_fnbody(fnbody, out);
}

pub  fn compile_case(var: Var, bodys: Vec<FnBodyRC>, out : &mut File)  {

}


pub fn compile_program(prog: ProgramRC, out : &mut File)  {
    let ProgramRC::Program(fun_dec) = prog;
    for (cste, fun) in fun_dec {
        let Const::Const(nom) = cste;
        write_ln(&format!("(func ${nom} "), out);
        compile_fn(fun, out);
        write_ln(&format!("export \"{nom}\" (func ${nom}))"), out);
    }
    
}

pub fn compile_fn(fun:FnRC, out:&mut File){
    let FnRC::Fn(params, fnbody) = fun;
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

pub fn compile_inc(var: Var, fnbody:FnBodyRC, out : &mut File)  {
    
    compile_fnbody(fnbody, out);
}

pub fn compile_dec(var: Var, fnbody:FnBodyRC, out : &mut File)  {
    
    compile_fnbody(fnbody, out);
}


pub fn compile_reset(var: Var, out : &mut File)  {

}

pub fn compile_reuse(var: Var, ctor: i32, args: Vec<Var>, out: &mut File){

} 