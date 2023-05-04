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
    let mut temp_beta_l : HashMap<Var,char> = beta.get(&c).unwrap().into_iter()
    .zip(vars.clone().into_iter())
    .map(|(status,y)| (y,*status)).collect();

    let mut beta_bis = beta.clone(); 
    tail_call(fnbody.clone(), &mut temp_beta_l, &mut beta_bis);

    let beta_l : HashMap<Var,char> = beta_bis.get(&c).unwrap().into_iter()
    .zip(vars.clone().into_iter())
    .map(|(status,y)| (y,*status)).collect();

    FnRC::Fn(vars.clone(), o_moins(vars.clone(), C(fnbody, beta_l.clone(), beta_bis), beta_l))
}

pub fn o_plus_var(x: Var, V: HashSet<Var>, F : FnBodyRC, beta_l : HashMap<Var,char>) -> FnBodyRC {
    if beta_l.get(&x).unwrap_or(&'O').eq(&'O') &&  !V.contains(&x) {
        F
    } else {
        FnBodyRC::Inc(x, Box::new(F))
    }
}

pub fn o_moins_var(x: Var, F : FnBodyRC, beta_l : HashMap<Var,char>) -> FnBodyRC {
    if beta_l.get(&x).unwrap_or(&'O').eq(&'O') &&  !FV(F.clone()).contains(&x) {
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
pub fn FV(F : FnBodyRC) -> HashSet<Var>{
    match F {
        FnBodyRC::Ret(x) => [x].into_iter().collect(),
        FnBodyRC::Let(x, e, fnbody) => {
            vec![x].into_iter().chain(FV_e(e)).chain(FV(*fnbody)).collect()
        },
        FnBodyRC::Case(x, bodys) => {
            vec![x].into_iter().chain(bodys.into_iter().map(|f| FV(f)).flatten()).collect()
        },
        FnBodyRC::Inc(x, fnbody) => {
            let mut res = FV(*fnbody);
            res.insert(x);
            res
        },
        FnBodyRC::Dec(x, fnbody) => {
            let mut res = FV(*fnbody);
            res.insert(x);
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
                    let o = vec!['O'; vars.clone().len()];
                    C_app(vars, beta.get(&c).unwrap_or(&o).clone(), FnBodyRC::Let(x, e,
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
                    if beta_l.clone().get(&y).unwrap_or(&'O').eq(&'B') {
                        let mut temp_beta_l = beta_l;
                        temp_beta_l.insert(x.clone(), 'B');

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


            FnBodyRC::Case(x.clone(), cases.into_iter()
            .map(|f| {
                let mut fv : Vec<Var> = FV(f.clone()).into_iter().collect();
                if !fv.contains(&x) {
                    fv.insert(0, x.clone())
                }
                o_moins(fv, C(f,beta_l.clone(), beta.clone()), beta_l.clone())
        })
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
        (None, Some(_)) | (Some(_), None) => {
            panic!("La liste des status et celle des variables n'ont pas la même taille {:?} {:?}.", status_var, vars)
        },
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
        (Some(y), Some(c)) => panic!("status de {:?} est différent de B ou O: {}", *y, *c),

    }
        },
        _ => panic!("Traitement non définis {:?}", fnbody),
    }

}


pub fn tail_call(fnbody : FnBodyRC, beta_l : &mut HashMap<Var,char>, beta: &mut HashMap<Const,Vec<char>>)  {
    match fnbody {
        FnBodyRC::Ret(_) => (),
        FnBodyRC::Let(y, e, F) => {
            match e {
                ExprRC::FnCall(c, vars) => {
                    match *F.clone() {
                        FnBodyRC::Ret(r) => {
                            if r.eq(&y) {
                                let update : Vec<char>= beta.get(&c)
                                .unwrap_or(&vec!['O'; vars.clone().len()])
                                .into_iter().zip(vars.into_iter())
                                .map(|(status, x)| {
                                    if beta_l.get(&x).unwrap_or(&'O').eq(&'O') {
                                        'O'
                                    } else {
                                        *status
                                    }
                                }).collect();
                                let _ = beta.insert(c, update);
                            }
                            return ();
                        },
                        _ => (),
                    }
                },
                ExprRC::Proj(_, x) => {
                    if beta_l.get(&x).unwrap_or(&'O').eq(&'B') {
                        beta_l.insert(y, 'B');
                    }
                },
                _ => (),
            }
            tail_call(*F, beta_l, beta)

        },
        FnBodyRC::Case(_, cases) => {
            for case in cases {
                tail_call(case, beta_l, beta)
            }
        },
        FnBodyRC::Inc(_, F) => tail_call(*F, beta_l, beta),
        FnBodyRC::Dec(_, F) => tail_call(*F, beta_l, beta),
    }
}
#[cfg(test)]
mod  tests {
    use std::collections::{HashMap, HashSet};
    use std::fs;

    use chumsky::Parser;

    use crate::compiler::ast_rc::{FnBodyRC, ProgramRC, FnRC};
    use crate::ast::{Var};
    use crate::compiler::inc::{insert_inc, tail_call};
    use crate::compiler::inferring::inferring_program;
    use crate::compiler::{reader_rc, Const};

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
        use super::{HashMap};
        use super::{Var, FnBodyRC};

        use crate::compiler::Const;
        use crate::compiler::ast_rc::ExprRC;
        use crate::compiler::inc::{C, C_app};
        
        #[test]
        fn test_c_app_no_vars() {
            let vars = vec![];
            let status_var = vec![];

            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(z.clone()));
            let fn_body = FnBodyRC::Let(z, ExprRC::Num(2), retour);

            let beta_l = HashMap::new();
            assert_eq!(C_app(vars, status_var, fn_body.clone(), beta_l), fn_body);
        }

        #[test]
        fn test_c_app_one_var_owned() {
            let z = Var::Var(String::from("z"));
            let vars = vec![z.clone()];
            let status_var = vec!['O'];

            let retour = Box::new(FnBodyRC::Ret(z.clone()));
            let fn_body = FnBodyRC::Let(z.clone(), ExprRC::Num(2), retour);

            let beta_l = HashMap::new();
            let expected = FnBodyRC::Inc(z, Box::new(fn_body.clone()));
            assert_eq!(expected, C_app(vars, status_var, fn_body, beta_l));
        }

        #[test]
        fn test_c_app_one_var_borrowed_dead_variable() {
            let x = Var::Var("x".to_owned());
            let vars = vec![x.clone()];
            let status_var = vec!['B'];

            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(z.clone()));
            let fn_body = FnBodyRC::Let(z.clone(), ExprRC::Num(2), retour.clone());

            let beta_l = HashMap::new();
            let expected = FnBodyRC::Let(z, ExprRC::Num(2), Box::new(
                FnBodyRC::Dec(x, retour)));
            assert_eq!(expected, C_app(vars, status_var, fn_body, beta_l));
        }
        
        #[test]
        fn test_C_consuming_same_args_multiple_times() {
            let c = Const::Const(String::from("c"));
            let y = Var::Var(String::from("y"));
            let z = Var::Var(String::from("z"));
            let retour = Box::new(FnBodyRC::Ret(z.clone()));

            let mut beta = HashMap::new();
            beta.insert(c.clone(), vec!['O';2]);

            let mut beta_l = HashMap::new();
            beta_l.insert(y.clone(), 'O');

            let vars = vec![y.clone();2];
            let F = FnBodyRC::Let(z, ExprRC::FnCall(c, vars), retour);
            
            let res = C(F.clone(),beta_l,beta);
            let expected = FnBodyRC::Inc(y, Box::new(F));
            
            assert_eq!(expected, res);

        }

        #[test]
        fn test_C_proj_borrowed() {
            let f = Const::Const(String::from("f"));
            let x = Var::Var(String::from("x"));
            let r = Var::Var(String::from("r"));
            let e = ExprRC::Proj(1, x.clone());
            let retour = Box::new(FnBodyRC::Ret(r.clone()));

            let body = FnBodyRC::Let(r.clone(), e.clone(), retour.clone());
            let mut beta = HashMap::new();
            beta.insert(f.clone(), vec!['O']);

            let mut beta_l = HashMap::new();
            beta_l.insert(x.clone(), 'O');

            let expected = FnBodyRC::Let(r.clone(), e, Box::new(FnBodyRC::Inc(r, 
                Box::new(FnBodyRC::Dec(x, retour))))); 
            assert_eq!(expected, C(body, beta_l, beta));
        }
        

    }

    #[test]
    fn  test_insert_tail_call() {
        let file_path = "./examples/tail_call.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_rc::program().parse(file_contents).expect("can't parse");
        
        let f = Const::Const(String::from("f"));
        let ProgramRC::Program(fun_dec) = prog;
        let f_fn = fun_dec.get(&f).unwrap();
        let FnRC::Fn(_, fnbody) = f_fn;

        let mut beta : HashMap<Const,Vec<char>> = vec![(f.clone(), vec!['B'])]
            .into_iter().collect();
        let mut beta_l : HashMap<Var,char> = vec![(Var::Var(String::from("x")), 'B')]
        .into_iter().collect();

        let expected : HashMap<Const,Vec<char>> = vec![(f, vec!['O'])]
            .into_iter().collect();
        tail_call(fnbody.clone(), &mut beta_l, &mut beta);
        
        assert_eq!(expected, beta);
    }
    #[test]
    fn test_tail_call() {
        let file_path = "./examples/tail_call.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_rc::program().parse(file_contents).expect("can't parse");

        let file_path_rc = "./examples/tail_call_rc.pstl";
        let file_contents_rc = fs::read_to_string(file_path_rc)
            .expect(format!("unable to read file + {}", file_path_rc).as_str());
        let expected = reader_rc::program().parse(file_contents_rc).expect("can't parse");
        
        let beta : HashMap<Const,Vec<char>> = vec![(Const::Const(String::from("f")), vec!['B'])]
            .into_iter().collect();

        assert_eq!(expected, insert_inc(prog.clone(), inferring_program(prog)));
        
    }
    #[test]
    fn test_map() {
        let file_path = "./examples/map_reuse.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_rc::program().parse(file_contents).expect("can't parse");

        let file_path_inc = "./examples/map_inc.pstl";
        let file_contents_inc = fs::read_to_string(file_path_inc)
            .expect(format!("unable to read file + {}", file_path_inc).as_str());
        let expected = reader_rc::program().parse(file_contents_inc).expect("can't parse");

        let beta : HashMap<Const,Vec<char>> = vec![(Const::Const(String::from("map")), vec!['B', 'O'])]
        .into_iter().collect();
        assert_eq!(expected, insert_inc(prog, beta))

    }

    #[test]
    fn test_swap() {
        let file_path = "./examples/swap_reuse.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_rc::program().parse(file_contents).expect("can't parse");
        let beta : HashMap<Const,Vec<char>> = vec![(Const::Const(String::from("swap")), vec!['O'])]
            .into_iter().collect();  

        let file_path_inc = "./examples/swap_inc.pstl";
        let file_contents_inc = fs::read_to_string(file_path_inc)
            .expect(format!("unable to read file + {}", file_path_inc).as_str());
        let expected = reader_rc::program().parse(file_contents_inc).expect("can't parse");

        assert_eq!(expected, insert_inc(prog, beta)) 
    }

    #[test]
    fn go_forward() {
        let file_path = "./examples/goForward_reuse.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader_rc::program().parse(file_contents).expect("can't parse");
        let beta : HashMap<Const,Vec<char>> = vec![(Const::Const(String::from("goForward")), vec!['O'])]
            .into_iter().collect();

        let file_path_inc = "./examples/goForward_inc.pstl";
        let file_contents_inc = fs::read_to_string(file_path_inc)
            .expect(format!("unable to read file + {}", file_path_inc).as_str());
        let expected = reader_rc::program().parse(file_contents_inc).expect("can't parse");
        assert_eq!(expected, insert_inc(prog, beta))   
    }

}