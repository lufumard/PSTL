 //#![allow(dead_code)]

pub mod ast_rc;
pub mod inferring;
pub mod reuse;
#[allow(non_snake_case)]
pub mod inc;
pub mod reader_rc;

pub mod utils;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use crate::compiler::primitives::get_num;

use crate::ast::CONST_NUM;
use crate::ast::Program;
pub use crate::ast::Var;
pub use crate::ast::Expr;
pub use crate::ast::FnBody;
pub use crate::ast::Fn;
pub use crate::ast::Const;
pub use crate::ast::CONST_FALSE;
pub use crate::ast::CONST_TRUE;
pub use crate::ast::CONST_NIL;
pub use crate::ast::CONST_LIST;
use self::ast_rc::Either;
use self::ast_rc::ExprRC;
use self::ast_rc::FnBodyRC;
use self::ast_rc::FnRC;
use self::ast_rc::ProgramRC;
use self::inc::insert_inc;
use self::inferring::inferring_program;
use self::primitives::has_args;
use self::reuse::insert_reuse;


pub mod primitives;
use primitives::is_primitive;
use primitives::compile_fncall_primitive;

use self::primitives::write_ln;
use self::primitives::write_out;

pub fn write_runtime(out :&mut File) {
    fn wr(out :&mut File) {
        write_ln("    ;; références", out);
        write_ln("    i32.const 0 ;; 0", out);
        write_ln("    i32.load    ;; x", out);
        write_ln("    i32.const 1 ;; x 1", out);
        write_ln("    call $__set_ref", out);
    }

    fn wa1(out :&mut File){
        write_ln("    ;; stoque le nombre", out);
        write_ln("    i32.const 0 ;; 0", out);
        write_ln("    i32.load    ;; x", out);
        write_ln("    i32.const 8 ;; x ", out);
        write_ln("    i32.add     ;; (x+8)", out);
        write_ln("    local.get $a;; (x+8) a", out);
        write_ln("    i32.store   ;;", out);
    }

    fn wpr(out: &mut File){
        write_ln("    ;; préparation de la valeur de retour", out);
        write_ln("    i32.const 0 ;; 0", out);
        write_ln("    i32.load    ;; x", out);
    }
        
    //crée un constructeur sans argument en wat
    write_ln("(func $__make_no_arg (param $b i32) (result i32)", out);
    write_ln("    ;; true ou false ou nil", out);
    write_ln("    local.get $b", out);
    write_ln("    call $__init_type", out);
        wr(out);
        wpr(out);
    write_ln("    ;; mise à jour de memory[0]", out);
    write_ln("    i32.const 8         ;; x 8", out);
    write_ln("    call $__offset_next ;; x", out);
    write_ln(")", out);

    write_ln("(func $__init_type (param $t i32)", out);
    write_ln("    i32.const 0 ;; 0", out);
    write_ln("    i32.load    ;; x", out);
    write_ln("    local.get $t;; x t", out);
    write_ln("    i32.store   ;; ", out);
    write_ln(")", out);

    write_ln("(func $__offset_next (param $n i32)", out);
    write_ln("    ;; mise à jour de memory[0]", out);
    write_ln("    i32.const 0 ;; 0", out);
    write_ln("    i32.const 0 ;; 0 0", out);
    write_ln("    i32.load    ;; 0 x", out);
    write_ln("    local.get $n;; 0 x n", out);
    write_ln("    i32.add     ;; x 0 (x+n)", out);
    write_ln("    i32.store   ;;", out);
    write_ln(")", out);

    write_ln("(func $__set_ref (param $adr i32) (param $ref i32)", out);
    write_ln("    ;; mise à jour de memory[0]", out);
    write_ln("    local.get $adr ;; @x", out);
    write_ln("    i32.const 4    ;; @x 4", out);
    write_ln("    i32.add        ;; @refs", out);
    write_ln("    local.get $ref ;; @refs n", out);
    write_ln("    i32.store      ;;", out);
    write_ln(")", out);

    //crée un constructeur de nombre en wat
    write_ln("(func $__make_num (param $a i32) (result i32)", out);
    write_ln("    ;; stoque le type du constructeur", out);
    write_ln(&format!("    i32.const {CONST_NUM}"), out);
    write_ln("    call $__init_type", out);

    wr(out);
    wa1(out);
    wpr(out);
    write_ln("    ;; mise à jour de memory[0]", out);
    write_ln("    i32.const 12        ;; x 12", out);
    write_ln("    call $__offset_next ;; x", out);
    write_ln(")", out);

    // crée un constructeur de liste
    write_ln("(func $__make_list (param $a i32) (param $b i32) (result i32)", out);
    write_ln("    ;; stoque le type du constructeur", out);
    write_ln(&format!("    i32.const {CONST_LIST}"), out);
    write_ln("    call $__init_type", out);
        wr(out);
        wa1(out);
    write_ln("    ;; stoque la deuxième adresse", out);
    write_ln("    i32.const 0 ;; 0", out);
    write_ln("    i32.load    ;; x", out);
    write_ln("    i32.const 12;; x 12", out);
    write_ln("    i32.add     ;; (x+12)", out);
    write_ln("    local.get $b;; (x+12) b", out);
    write_ln("    i32.store   ;;", out);
        wpr(out);
    write_ln("    ;; mise à jour de memory[0]", out);
    write_ln("    i32.const 16        ;; x 16", out);
    write_ln("    call $__offset_next ;; x", out);
    write_ln(")", out);

    write_ln("(func $__reset (param $var i32) (result i32)", out);
    compile_dec_body(Var::Var("var".to_string()), out);
    get_ref_loc(Var::Var("var".to_string()), out);
    write_ln("    i32.load", out);

    write_ln("    i32.eqz", out);
    write_ln("    if", out);
    write_ln("        i32.const 0", out);
    write_ln("        return", out);
    write_ln("    end", out);
    write_ln("    local.get $var", out);
    write_ln(")", out);
}

pub fn make_false(out:&mut File) {
    write_ln(&format!("i32.const {CONST_FALSE}"), out);
    write_ln("call $__make_no_arg", out);
}

pub fn make_true(out:&mut File) {
    write_ln(&format!("i32.const {CONST_TRUE}"), out);
    write_ln("call $__make_no_arg", out);
}

pub fn make_nil(out:&mut File) {
    write_ln(&format!("i32.const {CONST_NIL}"), out);
    write_ln("call $__make_no_arg", out);
}

pub  fn make_list(out:&mut File) {
    write_ln("call $__make_list", out);
}


pub  fn make_num(out:&mut File) {
    write_ln("call $__make_num", out);
}


pub fn compile(program: Program, out : &mut File){
    write_ln("(module", out);
    write_ln("(memory (import \"js\" \"mem\") 1)", out);
    write_runtime(out);
    let prog_reuse = insert_reuse(program);
    let beta: HashMap<Const,Vec<char>> = inferring_program(prog_reuse.clone());
    let prog_inc = insert_inc(prog_reuse, beta);
    compile_program(prog_inc, out);
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


pub fn compile_fncall(ident: Const, vars:Vec<Var>, out : &mut File)  {   
    let nom = string_of_const(ident);
    
    if is_primitive(&nom) {
        if has_args(&nom, vars.len()) == 0{
            compile_fncall_primitive(nom, vars, out);
        } else {
            panic!("Pas le bon nombre d'arguments sur l'appel de {nom}");
        }
    } else {
        for var in vars {
            compile_var(var, out);
        }
        write_ln(&format!("call $fun_{nom}"), out);
    }
}



pub fn compile_ctor(n: i32, vars:Vec<Var>, out : &mut File)  {
    match n {
        CONST_FALSE => make_false(out),
        CONST_TRUE  => make_true(out),
        CONST_NIL   => make_nil(out),
        CONST_LIST  => {
            assert_eq!(vars.len(), 2);
            compile_var(vars[0].to_owned(), out);
            compile_var(vars[1].to_owned(), out);
            make_list(out);
        },
        CONST_NUM   => {
            assert_eq!(vars.len(), 1);
            get_num(vars[0].to_owned(), out);
            make_num(out);
        },
        _ => panic!("Constructeur {n} inconnu")
    }
}

// On commence à 1
pub fn compile_proj(n: i32, var:Var, out : &mut File)  {
    compile_var(var, out);
    // calcul de l'offset en ajoutant la case des références et sur alignement des entier 32 bits
    let arg = (n + 1) * 4; 
    // sur liste : 3 4 123 456, proj1 => 123 (offset de 8) et proj2 => 456 (offset de 12)

    write_ln(&format!("i32.const {arg}"), out);
    write_ln("i32.add", out); // calcul de l'adresse à récupérer
    write_ln("i32.load", out) // chargement du nième argument
}

pub  fn compile_value(n: i32, out : &mut File)  {
    write_ln(&format!("i32.const {n}"), out);
    make_num(out); // création du nombre
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
    compile_expr(expr, out);
    /*if fnbody.clone() == FnBodyRC::Ret(var.clone()) {
        write_ln("return", out);
    } else {*/
        let v = string_of_var(var);
        write_ln(&format!("local.set ${v}"), out);
        compile_fnbody(fnbody, out);
    //}   
}

pub  fn compile_case(var: Var, bodys: Vec<FnBodyRC>, out : &mut File)  {
    for i in 0..bodys.len() {
        write_ln(&format!("(block $__case{i}"), out);
    } 
    compile_var(var, out);
    write_ln("i32.load", out);
    write_ln("(br_table ", out);
    for i in 0..bodys.len() {
        let n = bodys.len()-i-1;
        write_out(&format!("$__case{n} "), out);
    }
    write_ln(")", out);
    for body in bodys {
        write_ln(")", out);
        compile_fnbody(body, out);
    }    
}


pub fn compile_program(prog: ProgramRC, out : &mut File)  {
    let ProgramRC::Program(fun_dec) = prog;
    for (cste, fun) in fun_dec {
        let Const::Const(nom) = cste;
        write_out(&format!("(func $fun_{nom} (export \"{nom}\")"), out);
        compile_fn(fun, out);
        write_ln(")", out);
    }
}

pub fn compile_fn(fun:FnRC, out:&mut File){
    let FnRC::Fn(params, body) = fun;
    let mut vars : HashSet<String> = catch_var_names(body.clone());
    for param in params {
        let s = string_of_var(param);
        vars.remove(&s);
        write_out(&format!("(param ${s} i32) "), out);
    }
    write_ln("(result i32)", out);   
    
    for s in vars {
        write_ln(&format!("(local ${s} i32)"), out);
    }

    compile_fnbody(body, out);
}

fn catch_var_names(body : FnBodyRC) -> HashSet<String> {
    match body {
        FnBodyRC::Ret(_) => HashSet::new(),
        FnBodyRC::Let(var, _, body) => {
            let mut ns = HashSet::from([string_of_var(var), "__intern_var".to_string()]);
            for s in catch_var_names(*body){
                ns.insert(s);
            }
            return ns;
        },
        FnBodyRC::Case(_, bodys) => {
            let mut ns = HashSet::from(["__intern_var".to_string()]);
            for body in bodys {
                for s in catch_var_names(body){
                    ns.insert(s);
                }
            }
            return ns;
        },
        FnBodyRC::Inc(_, body) => catch_var_names(*body),
        FnBodyRC::Dec(_, body) => catch_var_names(*body),
    }
}

fn get_ref_loc(var: Var, out : &mut File) {
    compile_var(var, out);
    write_ln("i32.const 4", out);
    write_ln("i32.add", out);
}

pub fn compile_inc(var: Var, fnbody:FnBodyRC, out : &mut File)  {
    get_ref_loc(var.clone(), out);  // @ref
    get_ref_loc(var, out);          // @ref @ref
    write_ln("i32.load", out);   // @ref #ref
    write_ln("i32.const 1", out);// @ref #ref 1
    write_ln("i32.add", out);    // @ref #ref+1
    write_ln("i32.store", out);  // 
    compile_fnbody(fnbody, out);
}

fn compile_dec_body(var : Var, out : &mut File){
    get_ref_loc(var.clone(), out);  // @ref
    get_ref_loc(var, out);          // @ref @ref
    write_ln("i32.load", out);   // @ref #ref
    write_ln("i32.const 1", out);// @ref #ref 1
    write_ln("i32.sub", out);    // @ref #ref-1
    write_ln("i32.store", out);  // 
}

pub fn compile_dec(var: Var, fnbody:FnBodyRC, out : &mut File)  {
    compile_dec_body(var, out);
    compile_fnbody(fnbody, out);
}


pub fn compile_reset(var: Var, out : &mut File)  {
    compile_var(var, out);
    write_ln("call $__reset", out);
}

pub fn compile_reuse(var: Var, ctor: i32, args: Either<i32, Vec<Var>>, out: &mut File){
    compile_var(var.clone(), out);
    write_ln("i32.eqz", out);

    // soit types égaux
    compile_var(var.clone(), out);
    write_ln("i32.load", out);
    write_ln(&format!("i32.const {:?}", ctor.clone()), out);
    write_ln("i32.eq", out);

    // soit types tous les deux <= à 3
    compile_var(var.clone(), out);
    write_ln("i32.load", out);
    write_ln("i32.const 3", out);
    write_ln("i32.le_s", out);

    if ctor.clone() <= 3 {
        write_ln("i32.const 1", out);
    }else {
        write_ln("i32.const 0", out);
    }

    write_ln("i32.and", out);
    write_ln("i32.or", out);
    write_ln("i32.and", out);

    write_ln("if", out);
    match ctor {
        CONST_FALSE => make_false(out),
        CONST_TRUE => make_true(out),
        CONST_NIL => make_nil(out),
        CONST_LIST => match args.clone() {
            Either::Left(_) => panic!("i32 as args of ctor other than num"),
            Either::Right(vars) =>{
                compile_var(vars[0].clone(), out);
                compile_var(vars[1].clone(), out);
                make_list(out);
            },
        },
        CONST_NUM => match args {
            Either::Left(n) => compile_value(n, out),
            _ => panic!("vars as args of ctor other than num"),
        },
        _ => panic!("impossible")
    }
    write_ln("drop", out);
    write_ln("else", out);
    match ctor {
        CONST_NUM => match args {
            Either::Left(n) => {
                compile_reuse_no_arg(var.clone(), CONST_NUM, out);
                compile_var(var.clone(), out);
                write_ln("i32.load", out);
                write_ln("i32.const 8", out);
                write_ln("i32.add", out);
                write_ln(&format!("i32.const {n}"), out);
                write_ln("i32.store", out);
            },
            _ => panic!("vars as args of ctor other than num"),
        },
        CONST_LIST => match args {
                Either::Left(_) => panic!("i32 as args of ctor other than num"),
                Either::Right(vars) => {
                    compile_reuse_no_arg(var.clone(), CONST_LIST, out);
                    compile_var(var.clone(), out);
                    write_ln("i32.load", out);
                    write_ln("i32.const 8", out);
                    write_ln("i32.add", out);
                    compile_var(vars[0].to_owned(), out);
                    write_ln("i32.store", out);
                    compile_var(var.clone(), out);
                    write_ln("i32.const 12", out);
                    write_ln("i32.add", out);
                    compile_var(vars[1].to_owned(), out);
                    write_ln("i32.store", out);
                },
            },
        _ => compile_reuse_no_arg(var.clone(), ctor, out),
    }
    write_ln("end", out);
    compile_var(var, out);
}


fn compile_reuse_no_arg (var:Var, ctor:i32, out: &mut File) {
    compile_var(var.clone(), out);
    write_ln(&format!("i32.const {ctor}"), out);
    write_ln("i32.store", out);
    compile_var(var, out);
    write_ln("i32.const 4", out);
    write_ln("i32.add", out);
    write_ln("i32.const 1", out);
    write_ln("i32.store", out);
}