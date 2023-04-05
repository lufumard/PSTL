#![allow(dead_code)]

#[path = "../ast.rs"]
pub mod ast;

use std::collections::HashMap;
use std::fs::File;
use crate::ast::CONST_NUM;
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
use primitives::has_args;


pub mod primitives;
use primitives::is_primitive;
use primitives::compile_fncall_primitive;

use self::primitives::write_ln;



pub fn make_false() {

}

pub fn make_true() {

}

pub fn make_nil() {

}

pub  fn make_list(args : Vec<String>) {

}

pub  fn make_num(num : i32) {

}


pub fn compile(parsed : Vec<AST>, out : &File){

    write_ln("(module", out);
    write_ln("(table $table 1024 anyfunc)", out);
    for func in parsed {
        compile_ast(func, out);
    }
    write_ln("(export \"table\" (table $table))", out);

}



pub  fn create_ctor(n: i32, args: Vec<String>) {
    match n {
        CONST_FALSE => todo!(),
        CONST_TRUE => todo!(),
        CONST_NIL => todo!(),
        CONST_LIST => todo!(),
        _ => panic!("Contructeur non connu"),
    }
}

// Ast evaluation
pub  fn compile_ast(ast: AST, out : &File) {
    match ast {
        AST::Fn(fun) => compile_fun(fun, out),
        AST::Var(var) => compile_var(var, out),
        AST::Expr(expr) => compile_expr(expr, out),
        AST::FnBody(body) => compile_fnbody(body, out),
        AST::Const(_) => panic!("Impossible d'évaluer un Const"),
        AST::Program(c, fun) => compile_program(c, out),
    }
}

/*
* Var evaluation section
*/
pub fn compile_var(var: Var, out : &File) {
    let Var::Var(s) = var;

}

/*
* Expr evaluation section
*/

pub fn compile_expr(expr: Expr, out : &File) {
    match expr {
        Expr::FnCall(ident, vars) => compile_fncall(ident, out),
        Expr::Pap(ident, vars) => compile_pap(ident, out),
        Expr::Ctor(n, vars) => compile_ctor(n, out),
        Expr::Proj(n, var) => compile_proj(n, out),
        Expr::Num(n) => compile_value(n, out),
        Expr::PapCall(ident, var) => compile_pap_fncall(ident, out),
        Expr::Reset(_) => todo!(),
        Expr::Reuse(_, _, _) => todo!(),
    }
}

pub  fn compile_fun(fun:Fn, out : &File)  {
    let Fn::Fn(cst, _, _) = fun.clone(); 
    let Const::Const(name) = cst;
    lfn.insert(name.clone(), fun);
    Loc::Loc(0)
}


pub fn compile_fncall(ident: Const, out : &File)  {
    let Const::Const(nom) = ident.clone();
    if is_primitive(&nom) {
        let r = has_args(&nom, vars.len() as i32);
        if r == 0{
            let vars = vars.iter().map(|v| ct.get(v.to_owned())).collect();
            return compile_fncall_primitive(nom, out);
        } else if r < 0 {
            return compile_pap(ident, out);
        } else {
            panic!("Trop d'arguments");
        }
    }

    match lfn.get(&nom).cloned() {
        Some(Fn::Fn(_, args, body)) => compile_cons_fn(nom, args, body, out),
        None => {
            // Les appels partiels de variable de ne sont que sur un argument
            assert_eq!(vars.len(), 1);
            compile_pap_fncall(Var::Var(nom), vars[0].to_owned(), out)
        }
    }
}

pub fn compile_cons_fn(name:String, args:Vec<Var>, body:FnBody, vars:Vec<Var>, ct: & Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>)  {
    if args.len() == vars.len() {
        return compile_cons_full(args, body, out);
    } if args.len() > vars.len() {
        return compile_cons_part(Const::Const(name), out);
    }
    panic!("{} n'a pas autant d'arguments : attend {} args, en reçoit {}", name, args.len(), vars.len());
}

pub fn compile_cons_full(args_fun:Vec<Var>, body_fun:FnBody, args:Vec<Var>, ct: & Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>)  {
    let mut ctxt = ct.clone();
    for i in 0..args.len() {
        ctxt = ctxt.add(args_fun[i].to_owned(), ct.get(args[i].to_owned()));
    }
    return compile_fnbody(body_fun, &ctxt, h, lfn);
}

pub fn compile_cons_part(ident:Const, vars:Vec<Var>, ct: & Ctxt, h:&mut Heap, _:&mut HashMap<String, Fn>)  {
    let locs:Vec<Loc> = vars.iter().map(|v| ct.get(v.to_owned())).collect();
    let pap = Value::Pap(ident, locs);
    return h.add((pap, 1));
}

pub fn compile_pap_fncall(x: Var, out : &File)  {
    let (v, _) = h.get(ct.get(x));
    let l = ct.get(y.to_owned());
    match v {
        Value::Pap(c, mut vars) => {
            let Const::Const(name) = &c;
            vars.push(l);
            if is_primitive(&name) {
                let s = has_args(name, vars.len() as i32);
                if s == 0 {
                    return compile_fncall_primitive(name.to_owned(), out);
                } else if s < 1 {
                    let v = Value::Pap(Const::Const(name.to_string()), vars);
                    return h.add((v, 1));
                } else {
                    panic!("Pas le bon nombre d'arguments pour la primitive {}, reçois {}", name, vars.len() );
                }
                
            }else {  
                match lfn.get(name).cloned() {
                    Some(Fn::Fn(_, args, body)) => {
                        if args.len() == vars.len() {
                            compile_var_call_full(body, vars, args, out)
                        } else {
                            compile_var_call_part(c, out)
                        }
                    },
            
                    None =>  {
                        panic!("{} n'est pas une fonction", name)
                    } 
                }
            }
        },
        _ => panic!("Pas un pap"),
    }
}

pub fn compile_var_call_full(body: FnBody, out : &File)  {
    if fn_args.len() >= 1 {
        let mut ctx:Ctxt;
        ctx = c.add(fn_args[0].to_owned(), vars[0]);
        for i in 1..fn_args.len() {
            ctx = ctx.add(fn_args[i].to_owned(), vars[i]);
        }
        return compile_fnbody(body, &ctx, h, lfn);
    }else {
        return compile_fnbody(body, c, h, lfn);
    }
}

pub fn compile_var_call_part(ident: Const, out : &File)  {
    let v = Value::Pap(ident, vars);
    return h.add((v, 1));
}

pub  fn compile_pap(ident: Const, out : &File)  {
    let locs = vars.iter().map(|v| c.get(v.to_owned())).collect();
    let v = Value::Pap(ident, locs);
    return h.add((v, 1));
}



pub fn compile_ctor(n: i32, out : &File)  {
    let len:i32 = match vars.len().try_into() {
        Ok(v) => v,
        Err(_) => panic!("couldn't fit in i32"),
    };
    assert!(get_nb_args_ctor(n) == len);
    let locs : Vec<Loc> = vars.iter().map(|v| c.get(v.to_owned())).collect();
    let v = create_ctor(n, locs);
    return h.add((v, 1));
}

// On commence à 1
pub  fn compile_proj(n: i32, out : &File)  {
    assert!(n>0);
    let (v, _) = h.get(ct.get(var));

    if let Value::Ctor(_, locs) = v {
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("couldn't fit in usize"),
        };
        return locs[i-1].to_owned();
    } else {
        panic!("Proj sur autre qu'un constructeur")
    }
}

pub  fn compile_value(n: i32, out : &File)  {
    let num = make_num(n);
    return h.add((num, 1));
}

/*
* Fnbody evaluation section
*/
pub  fn compile_fnbody(body: FnBody, out : &File)  {
    match body {
        FnBody::Ret(var) => compile_ret(out),
        FnBody::Let(var, expr, fnbody) => compile_let(var, expr, *fnbody, out),
        FnBody::Case(var, bodys) => compile_case(var, bodys, out),
        FnBody::Inc(var, fnbody) => compile_inc(var, *fnbody, out),
        FnBody::Dec(var, fnbody) => compile_dec(var, *fnbody, out),
    }
}

pub  fn compile_ret(var: Var, out : &File)  {
    return compile_var(out);
}

pub  fn compile_let(var: Var, out : &File)  {
    let value = compile_expr(expr, out);
    let new_ctxt = ct.add(var, value);
    return compile_fnbody(fnbody, &new_ctxt, h, lfn);
}

pub  fn compile_case(var: Var, out : &File)  {
    let (v, _) = h.get(ct.get(var));
    if let Value::Ctor(n, _) = v {
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("cannot fit i32 into usize"),
        };
        assert!(i < bodys.len());
        compile_fnbody(bodys[i].to_owned(), out)
    } else {
        panic!("Case sur autre qu'un constructeur")
    }
}


pub fn compile_program(c: Const, out : &File)  {
    let Const::Const(nom) = c;
    let Fn::Fn(c_, _, _) = fun.clone();
    lfn.insert(nom, fun);
    let pap = Value::Pap(c_, vec![]);
    h.add((pap, 0))
}


pub fn compile_inc(var: Var, out : &File)  {
    h.inc(ct.get(var));
    return compile_fnbody(fnbody, out);
}

pub fn compile_dec(var: Var, out : &File)  {
    h.dec(ct.get(var));
    return compile_fnbody(fnbody, out);
}
