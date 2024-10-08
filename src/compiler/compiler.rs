
pub mod ast_rc;
pub mod inferring;
pub mod reuse;
#[allow(non_snake_case)]
pub mod inc;

#[allow (dead_code)]
pub mod reader_rc;

pub mod utils;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::vec;


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
use crate::compiler::ast_rc::CONST_PAP;
use crate::primitives::get_primitive_names;
use self::ast_rc::ConstWrapper;
use self::ast_rc::Either;
use self::ast_rc::ExprRC;
use self::ast_rc::FnBodyRC;
use self::ast_rc::FnRC;
use self::ast_rc::ProgramRC;
use self::inc::insert_inc;
use self::inferring::inferring_program;
use crate::primitives::nb_args;
use self::reuse::insert_reuse;


use indexmap::IndexMap;
use crate::primitives::is_primitive;
use crate::primitives::compile_fncall_primitive;


const OPTIMIZE_NO_ARG : bool = true;

use std::io::Write;
pub fn write_out(s : &str, out : &mut File){
    write!(out, "{}", s).err();
}

pub fn write_ln(s : &str, out : &mut File){
    write_out(&format!("{s}\n"), out);
}


pub fn make_bool(out:&mut File) {
    write_ln("call $__make_no_arg", out);
}

pub fn make_false(out:&mut File) {
    write_ln(&format!("i32.const {CONST_FALSE}"), out);
    make_bool(out);
}

pub fn make_true(out:&mut File) {
    write_ln(&format!("i32.const {CONST_TRUE}"), out);
    make_bool(out);
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

pub fn string_of_var(Var::Var(s):&Var) -> &String {
    return s;
}

pub fn string_of_const(Const::Const(c):&Const) -> &String {
    return c;
}

pub fn get_num(var:&Var, out : &mut File) {
    let s = string_of_var(var);
    write_ln(&format!("local.get ${s}"), out);
    write_ln("i32.const 8", out);
    write_ln("i32.add", out);
    write_ln("i32.load", out);
}

pub fn get_bool(var:&Var, out : &mut File) {
    compile_var(var, out);
    write_ln("i32.load", out);
}

/**
* L'emplacement du nième argument du ctor
* commence à 1
*/
fn ctor_arg_loc(var:&Var, n:i32, out:&mut File) {
    if n < 1 {
        panic!("Les indices des arguments commencent à 1");
    }
    compile_var(var, out);
    let i = n * 4 + 4;
    write_ln(&format!("i32.const {i}"), out);
    write_ln("i32.add", out);
}

fn get_type(var:&Var, out:&mut File){
    compile_var(var, out);
    write_ln("i32.load", out);
}


fn get_ref_loc(var: &Var, out : &mut File) {
    compile_var(var, out);
    write_ln("i32.const 4", out);
    write_ln("i32.add", out);
}

fn pap_nb_args_fixed_loc(var:&Var, out : &mut File){
    let s = string_of_var(var);
    write_ln(&format!("local.get ${s}"), out);
    write_ln("i32.const 12", out);
    write_ln("i32.add", out);
}

fn pap_fn_id_loc(var:&Var, out : &mut File){
    let s = string_of_var(var);
    write_ln(&format!("local.get ${s}"), out);
    write_ln("i32.const 8", out);
    write_ln("i32.add", out);
}

/**
 * commence à 1
 */
fn pap_arg_loc(var:&Var, n:i32, out:&mut File) {
    if n < 1 {
        panic!("Les indices des arguments commencent à 1");
    }
    compile_var(var, out);
    let i = n * 4 + 12;
    write_ln(&format!("i32.const {i}"), out);
    write_ln("i32.add", out);
}

pub fn compile(program: Program, out : &mut File){
    let prog_reuse = insert_reuse(program, &vec![0, 0, 0, 2, 1]);
    let beta: HashMap<Const,Vec<char>> = inferring_program(prog_reuse.clone());
    let prog_inc = insert_inc(prog_reuse, beta);
    write_ln("(module", out);
    write_ln("(memory (import \"js\" \"mem\") 1)", out);
    write_ln("(global $__FALSE_LOC (mut i32) (i32.const 4))", out);
    write_ln("(global $__TRUE_LOC (mut i32) (i32.const 12))", out);
    write_ln("(global $__NIL_LOC (mut i32) (i32.const 20))", out);
    let ProgramRC::Program(fun_dec) = &prog_inc;
    let fn_desc = make_fun_desc(fun_dec);
    write_runtime(&fn_desc, out);
    //dbg!(&prog_inc);
    compile_program(&prog_inc, &fn_desc, out);
    write_ln(")", out);

}


#[derive(Debug, Clone, PartialEq)]
pub struct FnDesc {
    id : i32,
    name : String,
    nb_args : usize,
} 


fn make_fun_desc (map : &IndexMap<Const, FnRC>) -> IndexMap<Const, FnDesc> {
    let mut res = IndexMap::new();
    
    let index = get_primitive_names()
        .iter()
        .fold(0, |index, name| {
            res.insert(Const::Const(name.clone()), FnDesc{
                id : index,
                name: name.to_string(),
                nb_args: nb_args(name.as_str()),
            });
            return index+1;
        });
    
    map.iter().fold(index, |index, (cste, fun)| {
        let Const::Const(nom) = cste;
        let FnRC::Fn(params, _) = fun;
        if params.len() > 0 {
            res.insert(cste.to_owned(), FnDesc{
                id : index,
                name: nom.clone(),
                nb_args: params.len(),
            });
            return index+1;
        } else {
            return index;
        }
    });

    return res;
}

pub fn write_runtime(fn_desc : &IndexMap<Const, FnDesc>, out :&mut File) {
    let pap = &Var::Var("pap".to_string());
    let var = &Var::Var("var".to_string());
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

    // init memory
    write_ln("(func $__init_memory (export \"__init_memory\") ", out);

    if OPTIMIZE_NO_ARG {

        // init false, true et nil si optimisé

        // init false
        write_ln("    i32.const 4", out);
        write_ln("    global.set $__FALSE_LOC", out);

        // init true
        write_ln("    i32.const 12", out);
        write_ln("    global.set $__TRUE_LOC", out);
        write_ln("    global.get $__TRUE_LOC", out);
        write_ln("    i32.const 1", out);
        write_ln("    i32.store", out);

        // init true
        write_ln("    i32.const 20", out);
        write_ln("    global.set $__NIL_LOC", out);
        write_ln("    global.get $__NIL_LOC", out);
        write_ln("    i32.const 2", out);
        write_ln("    i32.store", out);

        // init case mémoire
        write_ln("    i32.const 0", out);
        write_ln("    i32.const 28", out);
        write_ln("    i32.store", out);

    } else {

        // init case mémoire
        write_ln("    i32.const 0", out);
        write_ln("    i32.const 4", out);
        write_ln("    i32.store", out);

    }

    write_ln(")", out);


    //crée un constructeur sans argument en wat
    write_ln("(func $__make_no_arg (param $b i32) (result i32)", out);
    write_ln("    (local $result i32)", out);

    let result = Var::Var("result".to_string());

    if OPTIMIZE_NO_ARG {

        write_ln("    ;; true ou false ou nil", out);

        write_ln("    (block $case_nil ", out);
        write_ln("    (block $case_true ", out);        
        write_ln("    (block $case_false ", out);

        write_ln("    local.get $b", out);
        write_ln(&format!("    (br_table $case_false $case_true $case_nil)"), out);
        write_ln("    )", out);
        
        // case false
        write_ln("    global.get $__FALSE_LOC", out);
        write_ln("    local.tee $result", out);
            compile_add_ref(&result, 1, out);
        write_ln("    return", out);
        write_ln("    )", out);
        
        // case true
        write_ln("    global.get $__TRUE_LOC", out);
        write_ln("    local.tee $result", out);
            compile_add_ref(&result, 1, out);
        write_ln("    return", out);
        write_ln("    )", out);
        
        // case nil
        write_ln("    global.get $__NIL_LOC", out);
        write_ln("    local.tee $result", out);
            compile_add_ref(&result, 1, out);
        write_ln("    return", out);

    } else {

        write_ln("    ;; true ou false ou nil", out);
        write_ln("    local.get $b", out);
        write_ln("    call $__init_type", out);
            wr(out);
            wpr(out);
        write_ln("    ;; mise à jour de memory[0]", out);
        write_ln("    i32.const 8         ;; x 8", out);
        write_ln("    call $__offset_next ;; x", out);

    }

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
    write_ln("    ;; mise à jour des ref", out);
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
    //write_ln("    (local $__intern_var i32)", out);
    write_ln("    local.get $var", out);
    write_ln("    call $__dec", out);
    get_ref_loc(var, out);
    write_ln("    i32.load", out);

    write_ln("    i32.eqz", out);
    write_ln("    if", out);
    write_ln("        i32.const 0", out);
    write_ln("        return", out);
    write_ln("    end", out);
    write_ln("    local.get $var", out);
    write_ln(")", out);

    
    // crée un constructeur de pap
    write_ln("(func $__make_pap (param $a i32) (result i32)", out);
    write_ln("    ;; stoque le type du constructeur", out);
    write_ln(&format!("    i32.const {CONST_PAP}"), out);
    write_ln("    call $__init_type", out);
          wr(out);
    write_ln("    ;; stoque l'id de la fonction", out);
         wa1(out);
    write_ln("    ;; stoque le nombre d'arguments", out);
    write_ln("    i32.const 0 ;; 0", out);
    write_ln("    i32.load    ;; x", out);
    write_ln("    i32.const 12;; x 12", out);
    write_ln("    i32.add     ;; (x+12)", out);
    write_ln("    i32.const 0 ;; (x+12) 0", out);
    write_ln("    i32.store   ;;", out);
         wpr(out);
    write_ln("    ;; mise à jour de memory[0]", out);
    write_ln("    i32.const 16        ;; x 16", out);
    write_ln("    local.get $a        ;; x 16 a", out);
    write_ln("    call $__nb_args     ;; x 16 nb_args", out);
    write_ln("    i32.const 4         ;; x 16 nb_args 4", out);
    write_ln("    i32.mul             ;; x 16 nb_args*4", out);
    write_ln("    i32.add             ;; x offset", out);
    write_ln("    call $__offset_next ;; x", out);
    write_ln(")", out);

    write_ln("(func $__nb_args (export \"__nb_args\") (param $id i32) (result i32)", out);
    for i in 0..fn_desc.len() {
        write_ln(&format!("    (block $__case{i}"), out);
    } 

    //on charge le type de la variable
    write_ln("    local.get $id", out);

    // br_table choisi un enbranchement selon la valeur du type de la variable
    // br renvoie à la fin du block indiqué, 
    // donc si on veut éxécuter la suite du code de block $__case1, il faut faire br $__case2

    write_ln("  (br_table ", out);
    for i in 0..fn_desc.len() {
        let n = fn_desc.len()-i-1;
        write_out(&format!("$__case{n} "), out);
    }
    write_ln(")", out);
    for (_, desc) in fn_desc {
        write_ln(")", out);
        let nb_args = desc.nb_args;
        write_ln(&format!("    i32.const {nb_args} "), out);
        write_ln("    return", out);
    }
    write_ln(")", out);

    write_ln("(func $__copy_pap (param $var i32) (result i32)", out);

    write_ln("(local $pap i32)", out);
    write_ln("(local $args_rest i32)", out);
    write_ln("(local $loc_arg_var i32)", out);
    write_ln("(local $loc_arg_pap i32)", out);
    write_ln("(local $arg i32)", out);
    // make new pap
    pap_fn_id_loc(var, out);
    write_ln("i32.load", out);
    write_ln("call $__make_pap", out);
    
    write_ln("local.set $pap", out);
    
    // copy nb_args
    pap_nb_args_fixed_loc(pap, out);
    pap_nb_args_fixed_loc(var, out);
    write_ln("i32.load", out);
    
    write_ln("local.tee $args_rest", out);
    write_ln("i32.store", out);
    
    // copy args
    pap_arg_loc(var, 1, out);
    write_ln("local.set $loc_arg_var", out);
    
    pap_arg_loc(pap, 1, out);
    write_ln("local.set $loc_arg_pap", out);
    
    write_ln("local.get $args_rest", out);
    write_ln("if", out);
    write_ln("(loop $set_arg", out);
    write_ln("    local.get $loc_arg_pap", out);
    write_ln("    local.get $loc_arg_var", out);
    write_ln("    i32.load", out);
    write_ln("    i32.store", out);
    
    write_ln("    local.get $loc_arg_var", out);
    write_ln("    i32.load", out);
    write_ln("    local.set $arg", out);
    compile_add_ref(&Var::Var(String::from("arg")), 1, out);

    write_ln("    (i32.add (local.get $loc_arg_pap) (i32.const 4))", out); 
    write_ln("    local.set $loc_arg_pap", out);
        
    write_ln("    (i32.add (local.get $loc_arg_var) (i32.const 4))", out); 
    write_ln("    local.set $loc_arg_var", out);
    
    write_ln("    (i32.sub (local.get $args_rest) (i32.const 1))", out);
    write_ln("    local.tee $args_rest", out);
    write_ln("    br_if $set_arg", out);
    write_ln(")", out);
    write_ln("end", out);
    compile_var(pap, out);
    write_ln("return", out);
    write_ln(")", out);

   
    write_ln("(func $__exec_pap (param $pap i32) (result i32)", out);
    write_ln("(local $p_0 i32)", out);
    write_ln("(local $p_1 i32)", out);
    for i in 0..=fn_desc.len() {
        //on crée un block pour chaque cas énuméré
        write_ln(&format!("(block $__case{i}"), out);
    }
    // on charge l'id de la pap
    pap_fn_id_loc(pap, out);
    write_ln("i32.load", out);
    // br_table choisi un enbranchement selon la valeur du type de la variable
    // br renvoie à la fin du block indiqué, 
    // donc si on veut éxécuter la suite du code de block $__case1, il faut faire br $__case2
    write_ln("br_table ", out);
    for i in 0..fn_desc.len() {
        let n = fn_desc.len()-1-i;
        write_out(&format!("$__case{n} "), out);
    }
    write_ln(")", out);
    for (_, desc) in fn_desc {
        write_ln(")", out);
        if is_primitive(&desc.name){
            for i in 0..desc.nb_args {
                let n = (i+1).try_into().unwrap();
                pap_arg_loc(pap, n, out);
                write_ln("i32.load", out);
                write_ln(&format!("local.tee $p_{i}"), out);
                compile_add_ref(&Var::Var(format!("p_{i}")), 1, out);
            }

            let mut vars = vec![Var::Var("p_0".to_string()), Var::Var("p_1".to_string())];
            vars.truncate(desc.nb_args);
            compile_fncall_primitive(&desc.name, &vars, out);
        } else {
            for i in 0..desc.nb_args {
                let n = (i + 1).try_into().unwrap();
                pap_arg_loc(pap, n, out);
                write_ln("i32.load", out);
                write_ln(&format!("local.tee $p_0"), out);
                compile_add_ref(&Var::Var(format!("p_0")), 1, out);
            }
            let name = &desc.name;
            write_ln(&format!("call $fun_{name}"), out);
        }
        write_ln("local.get $pap", out);
        write_ln("call $__dec", out);
        write_ln("return", out);
    }
    write_ln(")", out);

    write_ln("(func $__dec (param $var i32)", out);
    write_ln(" (local $args_left i32)", out);
    write_ln(" (local $arg i32)", out);
    
        compile_var(var, out); // @var
    write_ln(" if", out);   // @var != null AND #ref > 0
    
        compile_add_ref(var, -1, out);
        get_ref_loc(var, out); // @ref
    write_ln("  i32.load", out);   // #ref
    write_ln("  i32.eqz", out);   // #ref est 0

    write_ln("  if", out);   // alors
    
        get_type(var, out); // type
    write_ln(&format!("    i32.const {CONST_PAP}"), out);
    write_ln("    i32.eq", out); // est type PAP

    write_ln("    if", out);
        pap_nb_args_fixed_loc(var, out); // @#args
    write_ln("      i32.load", out);   // #args
    write_ln("      local.set $args_left", out); 
        pap_arg_loc(var, 1, out); // @arg1
    write_ln("      local.set $arg", out); 
    write_ln("      (block $dec_end", out);   
    write_ln("        (loop $dec_loop", out);   
    write_ln("          local.get $args_left", out);
    write_ln("          i32.eqz", out);
    write_ln("          br_if $dec_end", out);
    
    write_ln("          local.get $arg", out);
    write_ln("          i32.load", out);
    write_ln("          call $__dec", out);
    write_ln("          (i32.add (local.get $arg) (i32.const 4))", out);
    write_ln("          local.set $arg", out);
    
    write_ln("          (i32.sub (local.get $args_left) (i32.const 1))", out);
    write_ln("          local.set $args_left", out); // #args--
    
    write_ln("          br $dec_loop", out);
    write_ln("        )", out);
    write_ln("      )", out);
    write_ln("    end", out);

    write_ln("    local.get $var", out);
    write_ln("    i32.load", out); // type
    write_ln(&format!("    i32.const {CONST_LIST}"), out);
    write_ln("    i32.eq", out); // est type LIST

    write_ln("    if ;; si de type LIST", out);
    ctor_arg_loc(var, 1, out);
    write_ln("      i32.load   ;; @arg 1", out);
    write_ln("      call $__dec;; dec arg 1", out);
    ctor_arg_loc(var, 2, out);
    write_ln("      i32.load   ;; @arg 2", out);
    write_ln("      call $__dec;; dec arg 2", out);
    write_ln("    end", out);
    write_ln("  end", out);
    write_ln(" end", out);
    write_ln(")", out);
    
}


pub fn compile_program(prog: &ProgramRC, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File)  {
    let ProgramRC::Program(fun_dec) = prog;
    for (cste, fun) in fun_dec {
        let Const::Const(nom) = cste;
        write_out(&format!("(func $fun_{nom} (export \"{nom}\")"), out);
        compile_fn(fun, fn_desc, out);
        write_ln(")", out);
    }
}

pub fn compile_fn(fun:&FnRC, fn_desc : &IndexMap<Const, FnDesc>, out:&mut File){
    let FnRC::Fn(params, body) = fun;
    let mut vars : HashSet<String> = catch_var_names(body);
    for param in params {
        let s = string_of_var(&param);
        vars.remove(s);
        write_out(&format!("(param ${s} i32) "), out);
    }
    write_ln("(result i32)", out);   
    
    for s in vars {
        write_ln(&format!("(local ${s} i32)"), out);
    }

    compile_fnbody(body, fn_desc, out);
}

fn catch_var_names(body : &FnBodyRC) -> HashSet<String> {
    match body {
        FnBodyRC::Ret(_) => HashSet::new(),
        FnBodyRC::Let(var, _, body) => {
            let mut ns = HashSet::from([string_of_var(&var).to_owned(), "__intern_var".to_string()]);
            for s in catch_var_names(&*body){
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
        FnBodyRC::Inc(_, body) => catch_var_names(&*body),
        FnBodyRC::Dec(_, body) => catch_var_names(&*body),
    }
}

/*
* Var evaluation section
*/
pub fn compile_var(var: &Var, out : &mut File) {
    let v = string_of_var(var);
    write_ln(&format!("local.get ${v}"), out);
}

/*
* Expr evaluation section
*/

pub fn compile_expr(expr: &ExprRC, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File) {
    match expr {
        ExprRC::FnCall(ident, vars) => compile_fncall(ident, vars, out),
        ExprRC::Pap(cste, args) => compile_pap(cste, args, fn_desc, out),
        ExprRC::Ctor(n, vars) => compile_ctor(*n, vars, out),
        ExprRC::Proj(n, var) => compile_proj(*n, var, out),
        ExprRC::Num(n) => compile_value(*n, out),
        ExprRC::PapCall(pap, arg) => compile_papcall(pap, arg, out),
        ExprRC::Reset(var) => compile_reset(&var, out),
        ExprRC::Reuse(var, i, args) => compile_reuse(var, *i, args, out),
    }
}


pub fn compile_fncall(ident: &Const, vars:&Vec<Var>, out : &mut File)  { 
    write_ln("\n;;fncall", out);  
    let nom = string_of_const(ident);
    
    if is_primitive(&nom) {
        if nb_args(&nom) == vars.len(){
            compile_fncall_primitive(&nom, &vars, out);
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


pub fn compile_pap(ident_wrap: &ConstWrapper, vars:&Vec<Var>, fn_desc:&IndexMap<Const, FnDesc>, out : &mut File)  { 
    let ConstWrapper::ConstWrapper(_, ident) = ident_wrap;
    write_ln("\n;;pap", out);  
    
    match fn_desc.get(ident) {
        Some(desc) => {
            if vars.len() > desc.nb_args {
                panic!("Trop d'arguments dans la construction d'une pap");
            }

            let id = desc.id;
            write_ln(&format!("i32.const {id}"), out);
            write_ln("call $__make_pap", out);  
            write_ln("local.set $__intern_var", out); 
            let intern = Var::Var("__intern_var".to_string());
            if vars.len() > 0 {
                for i in 0..vars.len() {
                    // charge l'emplacement de l'argument
                    let n = (i+1).try_into().unwrap();
                    pap_arg_loc(&intern, n, out);
                    compile_var(&vars[i], out);
                    write_ln("i32.store", out);
                }

                pap_nb_args_fixed_loc(&intern, out);

                let nb = vars.len();
                write_ln(&format!("i32.const {nb}"), out);
                write_ln("i32.store", out);
            }
            

            compile_var(&intern, out);
        },
        None => {
            let nom = string_of_const(ident);
            panic!("La fonction {nom} n'existe pas")
        },
    }
}


pub fn compile_ctor(n: i32, vars:&Vec<Var>, out : &mut File)  {
    write_ln("\n;;ctor", out);
    match n {
        CONST_FALSE => make_false(out),
        CONST_TRUE  => make_true(out),
        CONST_NIL   => make_nil(out),
        CONST_LIST  => {
            assert_eq!(vars.len(), 2);
            compile_var(&vars[0], out);
            compile_var(&vars[1], out);
            make_list(out);
        },
        CONST_NUM   => {
            assert_eq!(vars.len(), 1);
            get_num(&vars[0], out);
            make_num(out);
        },
        _ => panic!("Constructeur {n} inconnu")
    }
}

// On commence à 1
pub fn compile_proj(n: i32, var:&Var, out : &mut File)  {
    write_ln("\n;;proj", out);
    // calcul de l'offset en ajoutant la case des références et sur alignement des entier 32 bits
    // sur liste : 3 4 123 456, proj1 => 123 (offset de 8) et proj2 => 456 (offset de 12)
    ctor_arg_loc(var, n, out);
    write_ln("i32.load", out) // chargement du nième argument
}

pub  fn compile_value(n: i32, out : &mut File)  {
    write_ln("\n;;num", out);
    write_ln(&format!("i32.const {n}"), out);
    make_num(out); // création du nombre
}

pub fn compile_papcall(var:&Var, arg:&Var, out : &mut File) {
    compile_var(var, out);
    write_ln("call $__copy_pap", out);
    
    write_ln("local.set $__intern_var", out);
    // une copie de la variable pap a été créée
    
    compile_var(var, out);
    write_ln("call $__dec", out);

    let __intern_var = &Var::Var("__intern_var".to_string());

    pap_nb_args_fixed_loc(__intern_var, out);
    pap_nb_args_fixed_loc(__intern_var, out);
    write_ln("i32.load", out);
    
    write_ln("i32.const 1", out);
    write_ln("i32.add", out);
    
    write_ln("i32.store", out);
    
    // nb_args ++

    //compile_add_ref(arg, 1, out);

    pap_nb_args_fixed_loc(__intern_var, out);
    write_ln("i32.load", out);
    
    write_ln("i32.const 4", out);
    write_ln("i32.mul", out);
    pap_nb_args_fixed_loc(__intern_var, out);
    write_ln("i32.add", out);
    //emplacement nouvel argument
    
    compile_var(arg, out);
    write_ln("i32.store", out);
    
    // si il y a tous les arguments, exec_pap
    pap_nb_args_fixed_loc(__intern_var, out);
    write_ln("i32.load", out);
    
    pap_fn_id_loc(__intern_var, out);
    write_ln("i32.load", out);
    write_ln("call $__nb_args", out);
    
    write_ln("i32.eq", out);
    
    write_ln("if", out);
    write_ln("    local.get $__intern_var", out);
    write_ln("    call $__exec_pap", out);
    write_ln("    local.set $__intern_var", out);
    write_ln("end", out);
    // retourne le nouvel objet pap
    write_ln("local.get $__intern_var", out);
}

/*
* Fnbody evaluation section
*/
pub  fn compile_fnbody(body: &FnBodyRC, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File)  {
    match body {
        FnBodyRC::Ret(var) => compile_ret(var, out),
        FnBodyRC::Let(var, expr, fnbody) => compile_let(var, expr, &*fnbody, fn_desc, out),
        FnBodyRC::Case(var, bodys) => compile_case(var, bodys, fn_desc, out),
        FnBodyRC::Inc(var, fnbody) => compile_inc(var, &*fnbody, fn_desc, out),
        FnBodyRC::Dec(var, fnbody) => compile_dec(var, &*fnbody, fn_desc, out),
    }
}

pub  fn compile_ret(var: &Var, out : &mut File)  {
    write_ln("\n;;ret", out);
    compile_var(var, out);
    write_ln("return", out);
}

pub  fn compile_let(var: &Var, expr: &ExprRC, fnbody:&FnBodyRC, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File)  {
    write_ln("\n;;let", out);
    compile_expr(expr, fn_desc, out);
    if fnbody == &FnBodyRC::Ret(var.to_owned()) {
        write_ln("return", out);
    } else {
        let v = string_of_var(var);
        write_ln(&format!("local.set ${v}"), out);
        compile_fnbody(fnbody, fn_desc, out);
    }   
}

pub  fn compile_case(var: &Var, bodys: &Vec<FnBodyRC>, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File)  {
    write_ln("\n;;case", out);
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
        compile_fnbody(body, fn_desc, out);
    }    
}

fn compile_add_ref(var:&Var, n:i32, out:&mut File) {
    compile_var(var, out);                      // @var
    get_ref_loc(var, out);                      // @var @ref
    write_ln("i32.load", out);               // @var #ref
    write_ln(&format!("i32.const {n}"), out);// @var #ref n
    write_ln("i32.add", out);                // @var #ref+n
    write_ln("call $__set_ref", out);
}

pub fn compile_inc(var: &Var, fnbody:&FnBodyRC, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File)  {
    write_ln("\n;;inc", out);
    compile_add_ref(var, 1, out);
    compile_fnbody(fnbody, fn_desc, out);
}


pub fn compile_dec(var: &Var, fnbody:&FnBodyRC, fn_desc : &IndexMap<Const, FnDesc>, out : &mut File)  {
    write_ln("\n;;dec", out);
    compile_var(var, out);
    write_ln("call $__dec", out);
    compile_fnbody(fnbody, fn_desc, out);
}


pub fn compile_reset(var: &Var, out : &mut File)  {
    write_ln("\n;;reset", out);
    compile_var(var, out);
    write_ln("call $__reset", out);
}

pub fn compile_reuse(var: &Var, ctor: i32, args: &Either<i32, Vec<Var>>, out: &mut File){
    write_ln("\n;;reuse", out);

    // @var != 0
    compile_var(var, out);
    
    // ET
    // types égaux
    get_type(var, out);
    write_ln(&format!("i32.const {ctor}"), out);
    write_ln("i32.eq", out);

    

    if ctor <= 3 {
        // types tous les deux <= à 3
        get_type(var, out); // type
        write_ln("i32.const 3", out);
        write_ln("i32.le_s", out);
        write_ln("i32.or", out);  // type var <= 3 OU types égaux
        write_ln("i32.and", out); // ET @var != 0
    } else {
        write_ln("i32.and", out); // types égaux ET @var != 0
    }

    // si on peut réutiliser l'emplacement mémoire
    write_ln("if", out);
    match ctor {
        CONST_NUM => match args {
            Either::Left(n) => {
                compile_reuse_no_arg(var, CONST_NUM, out);
                
                ctor_arg_loc(var, 1, out);
                write_ln(&format!("i32.const {n}"), out);
                write_ln("i32.store", out);
            },
            _ => panic!("vars as args of ctor other than num"),
        },
        CONST_LIST => match args {
                Either::Left(_) => panic!("i32 as args of ctor other than num"),
                Either::Right(vars) => {
                    compile_reuse_no_arg(var, CONST_LIST, out);
                    
                    ctor_arg_loc(var, 1, out);
                    compile_var(&vars[0], out);
                    write_ln("i32.store", out); // store vars[0] @ var+8

                    ctor_arg_loc(var, 2, out);
                    compile_var(&vars[1], out);
                    write_ln("i32.store", out); // store vars[1] @ var+12
                },
            },
        _ => compile_reuse_no_arg(var, ctor, out),
    }
    compile_var(var, out);
    write_ln("local.set $__intern_var", out);
    // sinon, si on doit reprendre un espace mémoire
    write_ln("else", out);

    match ctor {
        CONST_FALSE => make_false(out),
        CONST_TRUE => make_true(out),
        CONST_NIL => make_nil(out),
        CONST_LIST => match args {
            Either::Left(_) => panic!("i32 as args of ctor other than num"),
            Either::Right(vars) =>{
                compile_var(&vars[0], out);
                compile_var(&vars[1], out);
                make_list(out);
            },
        },
        CONST_NUM => match args {
            Either::Left(n) => compile_value(*n, out),
            _ => panic!("vars as args of ctor other than num"),
        },
        _ => panic!("impossible")
    }
    // on n'oublie pas de remettre l'adresse dans la variable
    write_ln("local.set $__intern_var", out);
    write_ln("end", out);
    // if ne doit rien laisser de plus sur la pile
    write_ln("local.get $__intern_var", out);
}


fn compile_reuse_no_arg (var:&Var, ctor:i32, out: &mut File) {
    compile_var(var, out);
    write_ln(&format!("i32.const {ctor}"), out);
    write_ln("i32.store", out);
    get_ref_loc(var, out);
    write_ln("i32.const 1", out);
    write_ln("i32.store", out);
}