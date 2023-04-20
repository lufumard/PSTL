use std::collections::{HashSet, HashMap};

use indexmap::IndexMap;

use super::{ast_rc::{FnBodyRC, ExprRC, ProgramRC, FnRC}, Var, Const};

pub fn insert_inc(prog: ProgramRC, beta: HashMap<Const,Vec<char>>) -> ProgramRC {
    let ProgramRC::Program(fun_dec) = prog;

    let mut fun_dec_in = IndexMap::new();
    for (cste, fun) in fun_dec {
        fun_dec_in.insert(cste.clone(), delta_rc(cste, fun, beta.clone()));
    }

    ProgramRC::Program(fun_dec_in)
}
pub fn delta_rc(c: Const, f: FnRC,beta: HashMap<Const,Vec<char>>) -> FnRC {
    let FnRC::Fn(ref vars, fnbody) = f; 
    let beta_l : HashMap<Var,char> = beta.get(&c).unwrap().into_iter()
    .zip(vars.clone().into_iter())
    .map(|(status,y)| (y,*status)).collect();

    FnRC::Fn(vars.clone(), o_moins(vars.clone(), C(fnbody, beta_l.clone(), beta), beta_l))
}

pub fn o_plus_var(x: Var, V: HashSet<Var>, F : FnBodyRC, beta_l : HashMap<Var,char>) -> FnBodyRC {
    if beta_l.get(&x).unwrap().eq(&'O') &&  !V.contains(&x) {
        F
    } else {
        FnBodyRC::Inc(x, Box::new(F))
    }
}

pub fn o_moins_var(x: Var, F : FnBodyRC, beta_l : HashMap<Var,char>) -> FnBodyRC {
    if beta_l.get(&x).unwrap().eq(&'O') &&  !FV(F.clone()).contains(&x) {
        FnBodyRC::Dec(x, Box::new(F))
    } else {
        F
    }
}

pub fn o_moins(vars: Vec<Var>, F : FnBodyRC, beta_l : HashMap<Var,char>) -> FnBodyRC {
    let mut temp_vars = vars.clone();
    let binding = temp_vars.clone();
    let head = binding.get(0);
    match head {
        Some(x) => {
            temp_vars.remove(0);
            o_moins(temp_vars, o_moins_var(x.clone(), F, beta_l.clone()), beta_l)
        },
        None => F,
    }
}

pub fn FV_e(e: ExprRC) -> Vec<Var> {
    match e {
        ExprRC::FnCall(_, vars) => vars,
        ExprRC::PapCall(x, y) => vec![x, y],
        ExprRC::Pap(_, vars) => vars,
        ExprRC::Ctor(_, vars) => vars,
        ExprRC::Proj(_, x) => vec![x],
        ExprRC::Num(_) => vec![],
        ExprRC::Reset(x) => vec![x],
        ExprRC::Reuse(x, _, vars) => {
            vec![x].into_iter().chain(vars.into_iter()).collect()
        },
    }
}
/*retourne les variables non mortes de F */
pub fn FV(F : FnBodyRC) -> Vec<Var>{
    match F {
        FnBodyRC::Ret(x) => vec![x],
        FnBodyRC::Let(x, e, fnbody) => {
            vec![x].into_iter().chain(FV_e(e)).chain(FV(*fnbody)).collect()
        },
        FnBodyRC::Case(x, bodys) => {
            vec![x].into_iter().chain(bodys.into_iter().map(|f| FV(f)).flatten()).collect()
        },
        FnBodyRC::Inc(x, fnbody) => {
            let mut res = FV(*fnbody);
            res.push(x);
            res
        },
        FnBodyRC::Dec(x, fnbody) => {
            let mut res = FV(*fnbody);
            res.push(x);
            res
        },
    }
}

pub fn C(fnbody : FnBodyRC, beta_l : HashMap<Var,char>, beta: HashMap<Const,Vec<char>>) -> FnBodyRC {
    match fnbody.clone() {
        FnBodyRC::Ret(x) => o_plus_var(x, HashSet::new(), fnbody, beta_l),
        FnBodyRC::Let(x, e, F) => {
            match e.clone() {
                ExprRC::FnCall(c, vars) => {
                    C_app(vars, beta.get(&c).unwrap().clone(), FnBodyRC::Let(x, e,
                        Box::new(C(*F, beta_l.clone(), beta.clone()))), beta_l)
                },
                ExprRC::PapCall(z, y) => {
                    C_app(vec![z, y], vec!['O'; 2], FnBodyRC::Let(x, e, 
                        Box::new(C(*F, beta_l.clone(), beta))), beta_l)
                },
                ExprRC::Pap(_, vars) => {
                    C_app(vars.clone(), vec!['O'; vars.len()], 
                    FnBodyRC::Let(x, e, Box::new(C(*F, beta_l.clone(), beta))), beta_l)
                },
                ExprRC::Ctor(_, vars) => {
                    C_app(vars.clone(), vec!['O'; vars.len()], 
                    FnBodyRC::Let(x, e,Box::new(C(*F, beta_l.clone(), beta))), beta_l)
                },
                ExprRC::Proj(_, y) => {
                    if beta_l.clone().get(&y).unwrap().eq(&'B') {
                        let mut temp_beta_l = beta_l;
                        temp_beta_l.insert(y, 'B');
                        FnBodyRC::Let(x, e, Box::new(C(*F, temp_beta_l, beta)))
                    } else {
                        FnBodyRC::Let(x.clone(), e,Box::new(FnBodyRC::Inc(x.clone(), 
                        Box::new(o_moins_var(y, C(*F, beta_l.clone(), beta), beta_l)))))
                    }
                    
                },
                ExprRC::Num(_) | ExprRC::Reset(_) => FnBodyRC::Let(x, e, Box::new(C(*F,beta_l, beta))),
                ExprRC::Reuse(_, _, vars) => {
                    C_app(vars.clone(), vec!['O'; vars.len()],
                     FnBodyRC::Let(x, e, Box::new(C(*F, beta_l.clone(), beta))), beta_l)
                },
            }
        },
        FnBodyRC::Case(x, cases) => {
            FnBodyRC::Case(x, cases.into_iter()
            .map(|f| o_moins(FV(fnbody.clone()), f.clone(), beta_l.clone()))
            .collect())            
        },
        _ => panic!("Les instructions inc et dec n'ont pas encore été insérées"),
    }
}

pub fn C_app(vars: Vec<Var>, status_var : Vec<char>, fnbody : FnBodyRC, beta_l : HashMap<Var,char>) -> FnBodyRC {
    match fnbody.clone() {
        FnBodyRC::Let(z, e, F) => {
            let mut temp_vars = vars.clone();
    let hd_vars = vars.get(0);
    let mut temp_status = status_var.clone();
    let hd_status = temp_status.get(0);
    match (hd_vars, hd_status) {
        (None, None) => fnbody,
        (None, Some(_)) | (Some(_), None) => panic!("La liste des status et celle des variables n'ont pas la même taille."),
        (Some(y), Some('O')) => {
            temp_status.remove(0);
            temp_vars.remove(0);
            o_plus_var(y.clone(), temp_vars.clone().into_iter().chain(FV(*F).into_iter()).collect(),
             C_app(temp_vars, temp_status, fnbody, beta_l.clone()), beta_l)
        },
        (Some(y), Some('B')) => {
            temp_status.remove(0);
            temp_vars.remove(0);
            C_app(temp_vars, temp_status, FnBodyRC::Let(z, e, 
                Box::new(o_moins_var(y.clone(), *F, beta_l.clone()))), beta_l)
        },
        (Some(y), Some(_)) => panic!("status de {:?} est différent de B ou O", *y),

    }
        },
        _ => panic!("Traitement non définis"),
    }

}

#[cfg(test)]
mod  tests {
    use std::collections::{HashMap, HashSet};

    use crate::compiler::ast_rc::FnBodyRC;
    use crate::ast::{Var};

    use super::FV;

    #[test]
    fn test_FV_ret() {
        let x = Var::Var(String::from("x"));
        let body = FnBodyRC::Ret(Var::Var(String::from("x")));

        let res = FV(body);
        assert!(res.contains(&x));
    }
    
    #[test]
    fn test_FV_inc() {
        let x = Var::Var(String::from("x"));
        let body = FnBodyRC::Inc(x.clone(), 
            Box::new(FnBodyRC::Ret(Var::Var(String::from("y")))));

        let res = FV(body);
        assert!(res.contains(&x));
    }

    #[test]
    fn test_FV_dec() {
        let x = Var::Var(String::from("x"));
        let body = FnBodyRC::Dec(x.clone(), 
            Box::new(FnBodyRC::Ret(Var::Var(String::from("y")))));

        let res = FV(body);
        assert!(res.contains(&x));
    }
    
    #[cfg(test)]
    pub mod tests_o {
        use crate::compiler::ast_rc::ExprRC;
        use crate::compiler::inc::{o_plus_var, o_moins_var, o_moins};

        use super::{HashMap, HashSet};
        use super::{Var, FnBodyRC};

        /*cas où x est une variable morte et owned */
        #[test]
        fn test_o_plus_var_dead_owned() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');
            let res = o_plus_var(x.clone(),HashSet::new(),body.clone(), beta_l);
            assert_eq!(body, res);
        }

        #[test]
        fn test_o_plus_var_dead_borrowed() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            let res = o_plus_var(x.clone(),HashSet::new(),body.clone(), beta_l);
            let excpected = FnBodyRC::Inc(x.clone(), Box::new(body));
            assert_eq!(excpected, res);
            
        }

        #[test]
        fn test_o_plus_var_live_owned() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');

            let mut V = HashSet::new();
            V.insert(x.clone());

            let res = o_plus_var(x.clone(),V,body.clone(), beta_l);
            let excpected = FnBodyRC::Inc(x.clone(), Box::new(body));
            assert_eq!(excpected, res);
        }

        #[test]
        fn test_o_plus_var_live_borrowed() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');

            let mut V = HashSet::new();
            V.insert(x.clone());
            
            let res = o_plus_var(x.clone(),V,body.clone(), beta_l);
            let excpected = FnBodyRC::Inc(x.clone(), Box::new(body));
            assert_eq!(excpected, res);
        }

        /*cas où x est une variable morte et owned */
        #[test]
        fn test_o_moins_var_dead_owned() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("y")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');
            let res = o_moins_var(x.clone(),body.clone(), beta_l);
            let excpected = FnBodyRC::Dec(x.clone(), Box::new(body));
            assert_eq!(excpected, res);
        }

        /*cas où x est une variable morte et borrowed */
        #[test]
        fn test_o_moins_var_dead_borrowed() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("y")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            let res = o_moins_var(x.clone(),body.clone(), beta_l);
            assert_eq!(body, res);
            
        }

        #[test]
        fn test_o_moins_var_live_owned() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');

            let res = o_moins_var(x.clone(),body.clone(), beta_l);
            assert_eq!(body, res);
        }

        #[test]
        fn test_o_moins_var_live_borrowed() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            
            let res = o_moins_var(x.clone(),body.clone(), beta_l);
            assert_eq!(body, res);
        }

        /*Cas où la liste des variables est vide */
        #[test]
        fn test_o_moins() {
            let x = Var::Var(String::from("x"));
            let body = FnBodyRC::Ret(Var::Var(String::from("x")));

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            
            let res = o_moins(vec![], body.clone(), beta_l);
            assert_eq!(body, res);
        }
        
        #[test]
        fn test_moins_1() {
            let x = Var::Var(String::from("x"));
            let y = Var::Var(String::from("y"));
            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(x.clone()));

            let case1 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(0, Vec::new()), retour.clone());
            let case2 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(3, Vec::from([z.clone(),y.clone()])), retour.clone());
            let cases = vec![case1.clone(), case2.clone()];
            
            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');
            beta_l.insert(y.clone(), 'O');
            beta_l.insert(z.clone(), 'O');

            let body = FnBodyRC::Case(Var::Var("var".to_string()), cases);
            let res = o_moins(vec![x,y,z], body.clone(), beta_l);
            assert_eq!(body,res)
        }

        #[test]
        fn test_moins_2() {
            let x = Var::Var(String::from("x"));
            let y = Var::Var(String::from("y"));
            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(x.clone()));

            let case1 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(0, Vec::new()), retour.clone());
            let case2 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(3, Vec::from([z.clone(),y.clone()])), retour.clone());
            let cases = vec![case1.clone(), case2.clone()];
            
            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            beta_l.insert(y.clone(), 'B');
            beta_l.insert(z.clone(), 'B');

            let body = FnBodyRC::Case(Var::Var("var".to_string()), cases);
            let res = o_moins(vec![x,y,z], body.clone(), beta_l);
            assert_eq!(body,res)
        }

        #[test]
        fn test_moins_3() {
            let w = Var::Var(String::from("w"));
            let x = Var::Var(String::from("x"));
            let y = Var::Var(String::from("y"));
            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(x.clone()));

            let case1 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(0, Vec::new()), retour.clone());
            let case2 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(3, Vec::from([z.clone(),y.clone()])), retour.clone());
            let cases = vec![case1.clone(), case2.clone()];
            
            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            beta_l.insert(y.clone(), 'B');
            beta_l.insert(z.clone(), 'B');
            beta_l.insert(w.clone(), 'B');

            let body = FnBodyRC::Case(Var::Var("var".to_string()), cases);
            let res = o_moins(vec![x,y,z, w], body.clone(), beta_l);
            assert_eq!(body,res)
        }

        #[test]
        fn test_moins_4() {
            let w = Var::Var(String::from("w"));
            let x = Var::Var(String::from("x"));
            let y = Var::Var(String::from("y"));
            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(x.clone()));

            let case1 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(0, Vec::new()), retour.clone());
            let case2 = FnBodyRC::Let(x.clone(), ExprRC::Ctor(3, Vec::from([z.clone(),y.clone()])), retour.clone());
            let cases = vec![case1.clone(), case2.clone()];
            
            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');
            beta_l.insert(y.clone(), 'B');
            beta_l.insert(z.clone(), 'B');
            beta_l.insert(w.clone(), 'O');

            let body = FnBodyRC::Case(Var::Var("var".to_string()), cases);
            let res = o_moins(vec![x,y,z, w.clone()], body.clone(), beta_l);
            let expected = FnBodyRC::Dec(w, Box::new(body));
            assert_eq!(expected,res)
        }

    }

    #[cfg(test)]
    mod test_C {
        use super::{HashMap, HashSet};
        use super::{Var, FnBodyRC};

        use crate::compiler::inc::C;

        fn test_c_app_owned() {

        }

        fn test_c_app_borrwed() {
            
        }

        fn test_c_app() {
            
        }

        #[test]
        fn test_C_ret_borrowed() {
            /*let x = Var::Var(String::from("x"));
            let retour = Box::new(FnBodyRC::Ret(x.clone()));
            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'B');

            let excpected  = FnBodyRC::Inc(x.clone(), retour.clone());
            let res = C(*retour, beta_l, HashMap::new());

            assert_eq!(excpected, res);*/
        }

        #[test]
        fn test_C_ret_owned() {
            /*let x = Var::Var(String::from("x"));
            let retour = FnBodyRC::Ret(x.clone());
            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');

            let res = C(retour.clone(), beta_l, HashMap::new());


            assert_eq!(retour, res);*/
        }

        #[test]
        fn test_C_case() {

        }

        #[test]
        fn test_C_proj_borrowed() {

        }

        #[test]
        fn test_C_proj_owned() {

        }

        #[test]
        fn test_C_reset() {

        }

        #[test]
        fn test_C_fncall() {

        }

        #[test]
        fn test_C_pap() {

        }

        #[test]
        fn test_C_papcall() {

        }

        #[test]
        fn test_C_ctor() {

        }

        #[test]
        fn test_C_reuse() {
            
        }

    }

    #[test]
    fn test_map() {

    }

    #[test]
    fn test_swap() {
        
    }

}