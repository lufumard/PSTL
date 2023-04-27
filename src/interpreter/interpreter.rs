#![allow(dead_code)]

use std::collections::HashMap;
pub use crate::ast::CONST_NUM;
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
use primitives::has_args;


pub mod primitives;
use primitives::is_primitive;
use primitives::eval_fncall_primitive;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Loc{
    Loc(i32)
}

#[derive(Debug, Clone)]
pub enum Ctxt{
    Some {
        var:Var,
        loc:Loc,
        next:Box<Ctxt>,
    },
    None,
}

#[derive(Debug, Clone)]
pub struct Heap{
    map : HashMap<i32, Value>,
    loc : i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    Ctor (i32, Vec<Loc>),
    Pap (Const, Vec<Loc>),
    Null
}

impl Ctxt {
    pub fn get(&self, v:Var) -> Loc{
        let Var::Var(nom1) = &v;
        match self {
            Ctxt::Some { var, loc, next } => {  
                let Var::Var(nom2) = var;
                if *nom1 == *nom2 {
                    return loc.clone();
                }
                return next.get(v);
            }
            Ctxt::None => panic!("{} pas dans le ctxt", nom1),
        }
    }

    pub fn add(&self, v:Var, l:Loc) -> Ctxt{
        return Ctxt::Some{
            var: v, 
            loc: l, 
            next: Box::new(self.clone()),
        };
    }
}


impl Heap {
    pub fn get(&self, l:Loc) -> Value {
        let Loc::Loc(n) = l;
        match self.map.get(&n).cloned() {
            Some(r) => r,
            None => panic!("location {} not in heap", n),
        }
    }
    
    pub fn nb_alloc(&self) -> i32 {
        return self.loc.clone();
    }

    fn new_loc(& mut self) -> Loc {
        let n = self.loc;
        self.loc = n.clone()+1;
        Loc::Loc(n)
    }

    pub fn add(&mut self, v:Value) -> Loc {
        let l = self.new_loc();
        let Loc::Loc(n) = &l;
        self.map.insert(n.clone(), v);
        return l;
    }
}



pub  fn empty_ctxt() -> Ctxt {
    Ctxt::None
}

pub fn empty_heap() -> Heap {
    return Heap {map : HashMap::new(), loc : 1};
}


pub  fn make_false() -> Value {
    return Value::Ctor(CONST_FALSE, vec![]);
}

pub  fn make_true() -> Value {
    return Value::Ctor(CONST_TRUE, vec![]);
}

pub  fn make_nil() -> Value {
    return Value::Ctor(CONST_NIL, vec![]);
}

pub  fn make_list(args : Vec<Loc>) -> Value {
    assert!(args.len() == 2);
    return Value::Ctor(CONST_LIST, args);
}

pub  fn make_num(num : i32) -> Value {
    return Value::Ctor(CONST_NUM, vec![Loc::Loc(num)]);
}

pub  fn throw() {
    panic!("Evaluation non défini pour ce type");
}


pub fn start_interpreter (prog : Program, exec: Expr, ctxt: &Ctxt, heap: &mut Heap) -> Loc {
    let Program::Program(fun_dec) = prog;
    
    let mut liste_fun = HashMap::new();
    fun_dec.iter().fold(Loc::Loc(0), |_, (c, fun)| {
        let Const::Const(name) = c;
        liste_fun.insert(name.clone(), fun.clone());
        Loc::Loc(0)
    });
    //dbg!(&liste_fun);
    eval_expr(exec, ctxt, heap, &mut liste_fun)
}

pub fn debug_loc(loc : Loc, h : &mut Heap){
    let Loc::Loc(l) = loc.clone();
    let v = h.get(loc);
    match v {
        Value::Ctor(i, ls) => {
            match i {
                CONST_FALSE => println!("Loc : {l}; valeur : False"),
                CONST_TRUE => println!("Loc : {l}; valeur : True"),
                CONST_NIL => println!("Loc : {l}; valeur : Nil"),
                CONST_NUM => {
                    let Loc::Loc(n) = ls[0].to_owned();
                    println!("Loc : {l}; valeur : Num {n}");
                },
                CONST_LIST => {
                    let Loc::Loc(l1) = ls[0].to_owned();
                    let Loc::Loc(l2) = ls[1].to_owned();
                    println!("Loc : {l}; valeur : List (@{l1}, @{l2}), soit");
                    debug_loc(ls[0], h);
                    debug_loc(ls[1], h);
                },
                _ => println!("Loc : {l}; valeur : type {i} inconnu"),
            }
        },
        Value::Pap(c, ls) => {
            let Const::Const(nom) = c;
            let nb = ls.len();
            println!("Loc : {l}; valeur : Pap {nom}, {nb} arg:");
            ls.iter().map(|l| debug_loc(*l, h)).count();
        },
        Value::Null => println!("Loc : {l}; valeur : Null"),
    }
    
}

pub fn interpreter (program : Program, call : &String) {
    let mut heap = empty_heap();
    let ctxt = empty_ctxt();
    let exec = Expr::FnCall(Const::Const(call.to_owned()), vec![]);
    let res = start_interpreter (program, exec, &ctxt, &mut heap);

    let nb_alloc = heap.nb_alloc();
    
    println!("Attention ! L'interpréteur implémente le langage pur, sans inc, dec, reset, reuse");
    println!("Nombre d'allocations : {nb_alloc}");
    println!("Résultat : ");
    debug_loc(res, &mut heap);
}

pub  fn get_nb_args_ctor(n: i32) -> i32 {
    match n {
        CONST_FALSE => 0,
        CONST_TRUE => 0,
        CONST_NIL => 0,
        CONST_LIST => 2,
        CONST_NUM => 1,
        _ => panic!("Ctor {} non existant", n),
    }
}

pub  fn create_ctor(n: i32, args: Vec<Loc>) -> Value {
    match n {
        CONST_FALSE => make_false(),
        CONST_TRUE => make_true(),
        CONST_NIL => make_nil(),
        CONST_LIST => make_list(args),
        _ => panic!("Contructeur non connu"),
    }
}

/*
* Var evaluation section
*/
pub fn eval_var(var: Var, ct: &Ctxt, _:&mut Heap, _:&mut HashMap<String, Fn>) -> Loc {
    ct.get(var)
}

/*
* Expr evaluation section
*/

pub  fn eval_expr(expr: Expr, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    match expr {
        Expr::FnCall(ident, vars) => eval_fncall(ident, vars, ct, h, lfn),
        Expr::Pap(ident, vars) => eval_pap(ident, vars, ct, h, lfn),
        Expr::Ctor(n, vars) => eval_ctor(n, vars, ct, h, lfn),
        Expr::Proj(n, var) => eval_proj(n, var, ct, h, lfn),
        Expr::Num(n) => eval_value(n, ct, h, lfn),
        Expr::PapCall(ident, var) => eval_pap_fncall(ident, var, ct, h, lfn),
    }
}

pub  fn eval_fun(_:Fn, _: &Ctxt, _:&mut Heap, _:&mut HashMap<String, Fn>) -> Loc {
    panic!("Pas possible d'évaluer une fonction")
}


pub fn eval_fncall(ident: Const, vars: Vec<Var>, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    let Const::Const(nom) = ident.clone();
    if is_primitive(&nom) {
        let r = has_args(&nom, vars.len() as i32);
        if r == 0{
            let vars = vars.iter().map(|v| ct.get(v.to_owned())).collect();
            return eval_fncall_primitive(nom, vars, ct, h, lfn);
        } else if r < 0 {
            return eval_pap(ident, vars, ct, h, lfn);
        } else {
            panic!("Trop d'arguments");
        }
    }
    match lfn.get(&nom).cloned() {
        Some(Fn::Fn(args, body)) => eval_cons_fn(nom, args, body, vars, ct, h, lfn),
        None => {
            /*/
            // Les appels partiels de variable de ne sont que sur un argument
            assert_eq!(vars.len(), 1);
            println!("problème!");
            eval_pap_fncall(Var::Var(nom), vars[0].to_owned(), ct, h, lfn)
            */
            panic!("Fonction \"{nom}\" n'existe pas !")
        }
    }
}

pub fn eval_cons_fn(name:String, args:Vec<Var>, body:FnBody, vars:Vec<Var>, ct: & Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    if args.len() == vars.len() {
        return eval_cons_full(args, body, vars, ct, h, lfn);
    } if args.len() > vars.len() {
        return eval_cons_part(Const::Const(name), vars, ct, h, lfn);
    }
    panic!("{} n'a pas autant d'arguments : attend {} args, en reçoit {}", name, args.len(), vars.len());
}

pub fn eval_cons_full(args_fun:Vec<Var>, body_fun:FnBody, args:Vec<Var>, ct: & Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    let mut ctxt = ct.clone();
    for i in 0..args.len() {
        ctxt = ctxt.add(args_fun[i].to_owned(), ct.get(args[i].to_owned()));
    }
    return eval_fnbody(body_fun, &ctxt, h, lfn);
}

pub fn eval_cons_part(ident:Const, vars:Vec<Var>, ct: & Ctxt, h:&mut Heap, _:&mut HashMap<String, Fn>) -> Loc {
    let locs:Vec<Loc> = vars.iter().map(|v| ct.get(v.to_owned())).collect();
    let pap = Value::Pap(ident, locs);
    return h.add(pap);
}

pub fn eval_pap_fncall(x: Var, y: Var, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    let v = h.get(ct.get(x));
    let l = ct.get(y.to_owned());
    match v {
        Value::Pap(c, mut vars) => {
            let Const::Const(name) = &c;
            vars.push(l);
            if is_primitive(&name) {
                let s = has_args(name, vars.len() as i32);
                if s == 0 {
                    return eval_fncall_primitive(name.to_owned(), vars, ct, h, lfn);
                } else if s < 1 {
                    let v = Value::Pap(Const::Const(name.to_string()), vars);
                    return h.add(v);
                } else {
                    panic!("Pas le bon nombre d'arguments pour la primitive {}, reçois {}", name, vars.len() );
                }
                
            }else {  
                match lfn.get(name).cloned() {
                    Some(Fn::Fn(args, body)) => {
                        if args.len() == vars.len() {
                            eval_var_call_full(body, vars, args, ct, h, lfn)
                        } else {
                            eval_var_call_part(c, vars, ct, h, lfn)
                        }
                    },
            
                    None =>  {
                        panic!("\"{}\" n'est pas une fonction", name)
                    } 
                }
            }
        },
        _ => panic!("Pas un pap"),
    }
}

pub fn eval_var_call_full(body: FnBody, vars: Vec<Loc>, fn_args:Vec<Var>, c: &Ctxt, h:&mut Heap,  lfn:&mut HashMap<String, Fn>) -> Loc {
    if fn_args.len() >= 1 {
        let mut ctx:Ctxt;
        ctx = c.add(fn_args[0].to_owned(), vars[0]);
        for i in 1..fn_args.len() {
            ctx = ctx.add(fn_args[i].to_owned(), vars[i]);
        }
        return eval_fnbody(body, &ctx, h, lfn);
    }else {
        return eval_fnbody(body, c, h, lfn);
    }
}

pub fn eval_var_call_part(ident: Const, vars: Vec<Loc>, _: &Ctxt, h:&mut Heap,  _:&mut HashMap<String, Fn>) -> Loc {
    let v = Value::Pap(ident, vars);
    return h.add(v);
}

pub  fn eval_pap(ident: Const, vars: Vec<Var>, c: &Ctxt, h:&mut Heap,  _:&mut HashMap<String, Fn>) -> Loc {
    let locs = vars.iter().map(|v| c.get(v.to_owned())).collect();
    let v = Value::Pap(ident, locs);
    return h.add(v);
}



pub fn eval_ctor(n: i32, vars: Vec<Var>, c: &Ctxt, h:&mut Heap, _:&mut HashMap<String, Fn>) -> Loc {
    let len:i32 = match vars.len().try_into() {
        Ok(v) => v,
        Err(_) => panic!("couldn't fit in i32"),
    };
    assert!(get_nb_args_ctor(n) == len);
    let locs : Vec<Loc> = vars.iter().map(|v| c.get(v.to_owned())).collect();
    let v = create_ctor(n, locs);
    return h.add(v);
}

// On commence à 1
pub  fn eval_proj(n: i32, var: Var, ct: &Ctxt, h:&mut Heap, _:&mut HashMap<String, Fn>) -> Loc {
    assert!(n>0);
    let v = h.get(ct.get(var));

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

pub  fn eval_value(n: i32, _: &Ctxt, h:&mut Heap, _:&mut HashMap<String, Fn>) -> Loc {
    let num = make_num(n);
    return h.add(num);
}

/*
* Fnbody evaluation section
*/
pub  fn eval_fnbody(body: FnBody, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    match body {
        FnBody::Ret(var) => eval_ret(var, ct, h, lfn),
        FnBody::Let(var, expr, fnbody) => eval_let(var, expr, *fnbody, ct, h, lfn),
        FnBody::Case(var, bodys) => eval_case(var, bodys, ct, h, lfn),
    }
}

pub  fn eval_ret(var: Var, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    return eval_var(var, ct, h, lfn);
}

pub  fn eval_let(var: Var, expr: Expr, fnbody: FnBody, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    let value = eval_expr(expr, ct, h, lfn);
    let new_ctxt = ct.add(var, value);
    return eval_fnbody(fnbody, &new_ctxt, h, lfn);
}

pub  fn eval_case(var: Var, bodys: Vec<FnBody>, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    let v = h.get(ct.get(var));
    if let Value::Ctor(n, _) = v {
        let i:usize = match n.try_into() {
            Ok(v) => v,
            Err(_) => panic!("cannot fit i32 into usize"),
        };
        assert!(i < bodys.len());
        eval_fnbody(bodys[i].to_owned(), ct, h, lfn)
    } else {
        panic!("Case sur autre qu'un constructeur")
    }
}


pub fn eval_program(prog : Program, _: &Ctxt, _:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    let Program::Program(fun_dec) = prog;
    for (cste, fun) in fun_dec {
        let Const::Const(nom) = cste;
        lfn.insert(nom.clone(), fun);    
    }    
    return Loc::Loc(0);
}


/*
pub fn eval_inc(var: Var, fnbody: FnBody, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    h.inc(ct.get(var));
    return eval_fnbody(fnbody, ct, h, lfn);
}

pub fn eval_dec(var: Var, fnbody: FnBody, ct: &Ctxt, h:&mut Heap, lfn:&mut HashMap<String, Fn>) -> Loc {
    h.dec(ct.get(var));
    return eval_fnbody(fnbody, ct, h, lfn);
}

*/