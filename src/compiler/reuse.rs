#![allow(dead_code)]
use crate::compiler::ast_compiler::{FnBody, Var, Expr, Fn, Program};
use crate::compiler::ast_rc::{ExprRC, FnBodyRC, FnRC, ProgramRC};

use super::Const;
use super::ast_compiler::{CONST_FALSE, CONST_NIL, CONST_TRUE, CONST_LIST, CONST_NUM};
use super::ast_rc::ConstWrapper;
use super::utils::{is_in_expr, is_in_fn};


fn wrap_const(c : Const) -> ConstWrapper {
    let Const::Const(mut name) = c.clone();
    name.push_str("_c");
    ConstWrapper::ConstWrapper(Const::Const(name), c)
}

fn expr_pure_rc(e:Expr) -> ExprRC {
    match e {
        Expr::FnCall(ident, vars) => ExprRC::FnCall(ident, vars),
        Expr::PapCall(ident, var) => ExprRC::PapCall(ident, var),
        Expr::Pap(ident, vars) => ExprRC::Pap(wrap_const(ident), vars),
        Expr::Ctor(n, vars) => ExprRC::Ctor(n, vars),
        Expr::Proj(n, var) => ExprRC::Proj(n, var),
        Expr::Num(n) => ExprRC::Num(n),
    }
}

pub  fn get_nb_args_ctor(n: i32) -> usize {
    match n {
        CONST_FALSE => 0,
        CONST_TRUE => 0,
        CONST_NIL => 0,
        CONST_LIST => 2,
        CONST_NUM => 1,
        _ => panic!("Ctor {} non existant", n),
    }
}

#[derive(Clone,PartialEq,Debug)]
pub struct W {
    name: String,
    ident: i32
}

impl W {
    fn new(name: String, ident: i32) -> Self { Self { name, ident } }

    fn val(&self) -> Var {
        Var::Var(self.name.clone() + &self.ident.to_string())
    }

    fn get_and_inc(&mut self) -> Var{
        let temp = self.val();
        self.inc();
        temp
    }

    fn inc(&mut self) {
        self.ident +=  1;
    }
}

pub fn insert_reuse(prog: Program) -> ProgramRC {
    let Program::Program(fun_dec) = prog;
    ProgramRC::Program(fun_dec.into_iter()
    .map(|(_const, _fn)| (_const, delta_reuse(_fn))).collect())
}
pub fn delta_reuse(c : Fn) -> FnRC {
    match c {
        Fn::Fn(vars, fnbody) => FnRC::Fn(vars, R(fnbody)),
    }
}

#[allow(non_snake_case)]
pub fn R(body: FnBody) -> FnBodyRC {
    let mut w = W::new(String::from("w"), 1);
    match body {
        FnBody::Ret(x) => FnBodyRC::Ret(x),
        FnBody::Let(x, e, fnbody) => FnBodyRC::Let(x,expr_pure_rc(e), Box::new(R(*fnbody))),
        FnBody::Case(x, bodys) => {
            FnBodyRC::Case(x.clone(), ((*bodys.iter().enumerate().map(|(i, fi)| 
                D(x.clone(), get_nb_args_ctor(i.try_into().unwrap()), R(fi.clone()), & mut w)).collect::<Vec<FnBodyRC>>())).to_vec())
        } ,
    }
}

#[allow(non_snake_case)]
pub fn D(z:Var, n:usize,body:FnBodyRC, w: &mut W) -> FnBodyRC {
    match body {
        FnBodyRC::Ret(_) => body,
        FnBodyRC::Case(x, bodys) => {
            FnBodyRC::Case(x, bodys.into_iter().map(|f| D(z.clone(),n,f, w)).collect())
        },
        FnBodyRC::Let(ref x, ref e, ref fnbody) => {
            match is_in_expr(z.clone(), e.clone()) || is_in_fn(z.clone(), *fnbody.clone()) {
                true => FnBodyRC::Let(x.clone(), e.clone(), Box::new(D(z,n,*fnbody.clone(), w))),
                false => {
                    let temp = S(w.val(),n,body.clone());
                    match temp.clone() != body.clone() {
                        true => {
                            FnBodyRC::Let(w.get_and_inc(),ExprRC::Reset(z.clone()), Box::new(temp.clone()))
                        },
                        false => body,
                    }
                },
            }
        }
        _ => panic!("Les instructions reset et reuse n'ont pas encore été insérées")
    }
}

#[allow(non_snake_case)]
pub fn S(w:Var, n: usize, body:FnBodyRC) -> FnBodyRC {
    match body {
        FnBodyRC::Ret(_) => body,
        FnBodyRC::Case(var, bodys) => {
            FnBodyRC::Case(var, bodys.into_iter().map(|x| S(w.clone(),n,x)).collect())
        },
        FnBodyRC::Let(var, expr, fnbody) => match expr {
            ExprRC::Ctor(ident, ref vars) => if vars.len() == n {
                FnBodyRC::Let(var,ExprRC::Reuse(w,ident, vars.clone()), fnbody)
            } else {
                FnBodyRC::Let(var, expr, Box::new(S(w, n, *fnbody)))
            }
            ExprRC::Reset(_) | ExprRC::Reuse(_, _, _) => panic!("Les instructions reset et reuse n'ont pas encore été insérées"),
            _ => FnBodyRC::Let(var, expr, Box::new(S(w, n, *fnbody))),
        },
        FnBodyRC::Inc(_, fnbody) => S(w,n, *fnbody),
        FnBodyRC::Dec(_, fnbody) => S(w,n, *fnbody),
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::compiler::ast_rc::{ExprRC, FnBodyRC, ProgramRC, FnRC};
    use crate::compiler::ast_compiler::{Var, Expr, FnBody, Program};
    use crate::compiler::{reader_compiler, Const, Fn, CONST_LIST};
    use crate::compiler::reuse::{D,R,S,W, insert_reuse};
    use std::fs;
    use chumsky::Parser;
    use indexmap::IndexMap;

    #[test]
    fn test_S_ret() {
        let body = FnBodyRC::Ret(Var::Var(String::from("x")));
        assert_eq!(body, S(Var::Var(String::from("w")), 1, body.clone()))
    }

    /*Cas où n correspond aux nombre de paramètres du constructeur */    
    #[test]
    fn test_S_let_1() {
        let var = Var::Var(String::from("x"));
        let w = Var::Var(String::from("w"));
        let retour = Box::new(FnBodyRC::Ret(var.clone()));
        let body = FnBodyRC::Let(var.clone(),ExprRC::Ctor(0, Vec::new()) ,retour.clone());
        let expected = FnBodyRC::Let(var.clone(),ExprRC::Reuse(w.clone(), 0, Vec::new()) ,retour.clone());
        assert_eq!(expected, S(w.clone(), 0, body))
    }

    /*Cas où il n'ya pas de constructeur avec n paramètres */ 
    #[test]
    fn test_S_let_2() {
        let var = Var::Var(String::from("x"));
        let w = Var::Var(String::from("w"));
        let retour = Box::new(FnBodyRC::Ret(var.clone()));
        let body = FnBodyRC::Let(var.clone(),ExprRC::Ctor(0, Vec::new()) ,retour.clone());
        assert_eq!(body, S(w.clone(), 2, body.clone()))
    }

    #[test]
    fn test_S_case_1() {
        let w = Var::Var(String::from("w"));
        let x = Var::Var(String::from("x"));
        let y = Var::Var(String::from("y"));
        let z = Var::Var(String::from("z"));
        let retour = Box::new(FnBodyRC::Ret(x.clone()));

        let case1 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(0, Vec::new()), retour.clone());
        let case2 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(3, Vec::from([z.clone(),y.clone()])), retour.clone());
        let cases = vec![case1.clone(), case2.clone()];
        let body = FnBodyRC::Case(Var::Var("var".to_string()), cases);

        let case1_expected = FnBodyRC::Let(x.clone(),ExprRC::Reuse(w.clone(), 0, Vec::new()) ,retour.clone());
        let cases_expected = vec![case1_expected.clone(), case2.clone()];
        let expected = FnBodyRC::Case(Var::Var("var".to_string()), cases_expected);
        assert_eq!(expected ,S(Var::Var(String::from("w")),0,body.clone()))
    }

    /*Cas où il n'ya pas de constructeur avec n paramètres */ 
    #[test]
    fn test_S_case_2() {
        let v1 = Var::Var("v1".to_string());
        let v2 = Var::Var("v2".to_string());
        let cases = vec![FnBodyRC::Ret(v1.clone()), FnBodyRC::Ret(v2.clone())];
        let body = FnBodyRC::Case(Var::Var("var".to_string()), cases);
        assert_eq!(body ,S(Var::Var(String::from("w")),0,body.clone()))
    }

    #[test]
    fn test_D_ret() {
        let body = FnBodyRC::Ret(Var::Var(String::from("x")));
        let mut w = W::new(String::from("w"), 0);
        assert_eq!(body, D(Var::Var(String::from("w")), 1, body.clone(), &mut w))
    }

    #[test]
    fn test_D_let_dead_variable() {

    }

    #[test]
    fn test_D_let_not_dead_variable() {

    }

    #[test]
    fn test_D_case() {
        
    }

    /*Cas où on doit insérer un reuse */
    #[test]
    fn test_D_reuse() {

    }

    #[test]
    fn test_R_ret() {
        let body = FnBody::Ret(Var::Var(String::from("x")));
        let expected = FnBodyRC::Ret(Var::Var(String::from("x")));
        assert_eq!(expected, R(body));
    }

    #[test]
    fn tes_R_let() {
        
        let x = Var::Var(String::from("x"));
        let z = Var::Var(String::from("z")); 
        let retour = Box::new(FnBody::Ret(z.clone()));
        let body = FnBody::Let(z.clone(), Expr::Proj(0, x.clone()), retour);
        let expected = FnBodyRC::Let(z.clone(), ExprRC::Proj(0, x.clone()), 
            Box::new(FnBodyRC::Ret(Var::Var(String::from("z")))));
        assert_eq!(expected, R(body));
    }

    #[test]
    fn test_R_case() {
        let file_path = "./examples/map_pstl.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_compiler::program().parse(file_contents).expect("can't parse");
        let Program::Program(fun_dec) = prog;
        let fn_ = fun_dec.get(&Const::Const(String::from("map"))).unwrap();
        let  Fn::Fn(_, fnbody) = fn_;

        let x = Var::Var(String::from("x"));
        let y = Var::Var(String::from("y"));
        let s = Var::Var(String::from("s"));
        let xs = Var::Var(String::from("xs"));
        let ys = Var::Var(String::from("ys"));
        let r = Var::Var(String::from("r"));
        let w1 = Var::Var(String::from("w1"));

        let expected = FnBodyRC::Case(xs.clone(), vec![
            FnBodyRC::Ret(xs.clone()),
            FnBodyRC::Ret(xs.clone()),
            FnBodyRC::Ret(xs.clone()),
            FnBodyRC::Let(x.clone(), ExprRC::Proj(1, xs.clone()), 
            Box::new(FnBodyRC::Let(s.clone(), ExprRC::Proj(2, xs.clone()), 
            Box::new(FnBodyRC::Let(w1.clone(), ExprRC::Reset(xs.clone()), 
            Box::new(FnBodyRC::Let(y.clone(), ExprRC::FnCall(Const::Const(String::from("f")), vec![x.clone()]), 
            Box::new(FnBodyRC::Let(ys.clone(), ExprRC::FnCall(Const::Const(String::from("map")), vec![Var::Var(String::from("f")), s.clone()]),
            Box::new(FnBodyRC::Let(r.clone(), ExprRC::Reuse(w1.clone(), CONST_LIST, vec![y.clone(), ys.clone()]), 
            Box::new(FnBodyRC::Ret(r.clone()))))))))))))),
            FnBodyRC::Ret(xs.clone())]);

        assert_eq!(expected, R(fnbody.clone())) ;
    }

    #[test]
    fn test_reuse() {
        let file_path = "./examples/id_pair.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_compiler::program().parse(file_contents).expect("can't parse");

        let x = Var::Var(String::from("x"));
        let y = Var::Var(String::from("y")); 
        let ret_x = FnBodyRC::Ret(x.clone());
        let ret_y = FnBodyRC::Ret(y.clone());

        let mut fun_dec : IndexMap<Const, FnRC> = IndexMap::new();
        let id_fn = FnRC::Fn(vec![x.clone()], FnBodyRC::Ret(x.clone()));
        let fst_fn = FnRC::Fn(vec![x.clone(), y.clone()], ret_x.clone());
        let sec_fn = FnRC::Fn(vec![x.clone(), y.clone()], ret_y.clone());
        
        fun_dec.insert(Const::Const(String::from("id")), id_fn.clone());
        fun_dec.insert(Const::Const(String::from("fst")), fst_fn.clone());
        fun_dec.insert(Const::Const(String::from("sec")), sec_fn.clone());
        
        let expected : ProgramRC = ProgramRC::Program(fun_dec);
        assert_eq!(expected, insert_reuse(prog));
    }
    
}
