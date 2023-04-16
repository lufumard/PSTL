use std::collections::{HashSet, HashMap};

use super::{ast_rc::{FnBodyRC, ExprRC, ProgramRC, FnRC}, Var, Const};

pub fn insert_inc(prog: ProgramRC) -> ProgramRC {
    todo!()
}
pub fn delta_rc(c : FnRC) -> FnRC {
    todo!()
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
        F
    } else {
        FnBodyRC::Dec(x, Box::new(F))
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
        (None, Some(_)) | (Some(_), None) => todo!(),
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