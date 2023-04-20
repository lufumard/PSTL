use std::collections::HashSet;
use std::collections::HashMap;

use indexmap::IndexMap;

use crate::ast::{Var};
use crate::compiler::ast_rc::ExprRC;
use crate::compiler::ast_rc::FnBodyRC;
use crate::compiler::ast_rc::FnRC;

use super::Const;
use super::ast_rc::ConstWrapper;
use super::ast_rc::ProgramRC;



pub fn collect_o(fnbody:FnBodyRC, beta: HashMap<Const,Vec<char>>) -> HashSet<Var> {
    match fnbody {
        FnBodyRC::Ret(_) => HashSet::new(),
        FnBodyRC::Let(z, e, f) => match e {
            ExprRC::FnCall(c, vars) => {
                collect_o(*f, beta.clone())
                .union(&vars
                    .into_iter()
                    .enumerate()
                    .filter(|&(i, _)| beta.clone().get(&c).unwrap().get(i).unwrap().eq(&'O'))
                    .map(|(_, e)| e)
                    .collect::<HashSet<Var>>()).cloned().collect()
            },
            ExprRC::PapCall(x, y) => collect_o(*f, beta).union(&[x,y].into()).cloned().collect(),
            ExprRC::Pap(_, vars) => collect_o(*f, beta).union(&vars.into_iter().collect()).cloned().collect(),
            ExprRC::Proj(_, x) => {
                let temp = collect_o(*f, beta);
                if temp.contains(&z) {
                    temp.union(&[x].into()).cloned().collect()
                } else {
                    temp
                }
            },
            ExprRC::Num(_) => HashSet::new(),
            ExprRC::Reset(x) =>  collect_o(*f, beta).union(&[x].into()).cloned().collect(),
            _ =>  collect_o(*f, beta),
        },
        FnBodyRC::Case(_, bodys) =>  {
            bodys.into_iter()
            .map(|f| collect_o(f, beta.clone()))
            .collect::<Vec<HashSet<Var>>>().iter().flatten().cloned().collect::<HashSet<Var>>()
        },
        _ => panic!("Les instructions inc et dec n'ont pas encore été insérées")
    }
}

/*Retourne une map qui pour chaque paramètre de la fonction
définit si il est borrowed('B') ou owned('O')
 */
pub fn inferring_signatures(c: Const, f: FnRC,beta: HashMap<Const,Vec<char>>) -> Vec<char> {
    let mut temp_beta = beta.clone();

    let FnRC::Fn(ref vars, fnbody) = f;  
    let mut beta_c: Vec<char> = vars.into_iter().map(|_| 'B').collect();
    loop {
        temp_beta.insert(c.clone(), beta_c.clone());
        let s: HashSet<Var> = collect_o(fnbody.clone(), temp_beta.clone());
        let it = vars.iter().zip(beta_c.iter());
        let mut temp_beta_c: Vec<char> = Vec::new();
        for (var, beta_c_var) in it {
            if s.contains(&var) {
                temp_beta_c.push('O')
            } else {
                temp_beta_c.push(*beta_c_var)
            }
        }
        if temp_beta_c == beta_c {
            return  temp_beta_c;
        }
        beta_c = temp_beta_c
    }

}

pub fn inferring_pap_fnbody(fnbody:FnBodyRC, mut beta: HashMap<Const,Vec<char>>) -> HashMap<Const,Vec<char>> {
    match fnbody {
        FnBodyRC::Let(_, e, _body) => {
            match e {
                ExprRC::Pap(ConstWrapper::ConstWrapper(wrap,_), vars) => {
                    beta.insert(wrap, vec!['O'; vars.len()]).unwrap();
                    beta
                },
                _ => beta,
            }
        },
        _ => beta,
    }
}
pub fn inferring_pap(fun_dec : IndexMap<Const, FnRC>) -> HashMap<Const,Vec<char>>{
    let beta: HashMap<Const,Vec<char>> = HashMap::new();

    for (_, fun) in fun_dec {
        let FnRC::Fn(_, fnbody) = fun;
        beta.clone().extend(inferring_pap_fnbody(fnbody, beta.clone()));
        
    }

    beta
}

pub fn inferring_program(prog: ProgramRC) -> HashMap<Const,Vec<char>>{
    let ProgramRC::Program(fun_dec) = prog;
    let mut beta = inferring_pap(fun_dec.clone());

    for (cste, fun) in fun_dec {
        let beta_c = inferring_signatures(cste.clone(), fun, beta.clone());
        beta.insert(cste, beta_c);
    }

    beta
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests_collect_o {
    use std::collections::{HashSet, HashMap};

    use crate::ast::{Var, Const};
    use crate::compiler::ast_rc::{FnBodyRC, ExprRC, ConstWrapper};
    use crate::compiler::inferring::collect_o;

    #[test]
    fn  test_ctor() {
        let x = Var::Var(String::from("x"));
        let retour = Box::new(FnBodyRC::Ret(x.clone()));

        let body = FnBodyRC::Let(x.clone(), ExprRC::Ctor(0, Vec::new()), retour.clone());
        let expected: HashSet<Var> = HashSet::new();
        assert_eq!(expected, collect_o(body, HashMap::new()))
    }

    #[test]
    fn test_reset() {
        let x = Var::Var(String::from("x"));
        let z = Var::Var(String::from("z"));
        let retour = Box::new(FnBodyRC::Ret(z.clone()));
        let body = FnBodyRC::Let(z.clone(),ExprRC::Reset(x.clone()),retour.clone());
        let expected = HashSet::from([x.clone()]);
        assert_eq!(expected,collect_o(body, HashMap::new()));
    }

    #[test]
    fn test_reuse() {
        let x = Var::Var(String::from("x"));
        let w = Var::Var(String::from("w"));
        let retour = Box::new(FnBodyRC::Ret(x.clone()));
        let body = FnBodyRC::Let(x.clone(),ExprRC::Reuse(w.clone(), 0, Vec::new()) ,retour.clone());
        let expected: HashSet<Var> = HashSet::new();
        assert_eq!(expected, collect_o(body, HashMap::new()))
    }

    #[test]
    fn test_fn_call() {
        let x = Var::Var(String::from("x"));
        let y = Var::Var(String::from("y"));
        let id = Const::Const(String::from("id"));
        let retour = Box::new(FnBodyRC::Ret(x.clone()));

        let mut beta: HashMap<Const,Vec<char>> = HashMap::new();
        beta.insert(id.clone(), vec!['O']);

        let body = FnBodyRC::Let(x.clone(), ExprRC::FnCall(id.clone(), vec![y.clone()]), retour.clone());
        let expected = HashSet::from( [y.clone()]);
        assert_eq!(expected, collect_o(body, beta))

    }

    #[test]
    fn test_pap_call() {
        let x = Var::Var(String::from("x"));
        let z = Var::Var(String::from("z"));
        let y = Var::Var(String::from("y"));
        let retour = Box::new(FnBodyRC::Ret(z.clone()));

        let body = FnBodyRC::Let(z.clone(), ExprRC::PapCall(x.clone(), y.clone()), retour.clone());
        let expected = HashSet::from([x.clone(),y.clone()]);
        assert_eq!(expected, collect_o(body, HashMap::new()))
        
    }   

    #[test]
    fn test_pap() {
        let x = Var::Var(String::from("x"));
        let z = Var::Var(String::from("z"));
        let retour = Box::new(FnBodyRC::Ret(z.clone()));

        let add = Const::Const(String::from("add"));
        let add_wrap = ConstWrapper::ConstWrapper(Const::Const(String::from("add_c")), add.clone());
        let mut beta: HashMap<Const,Vec<char>> = HashMap::new();
        beta.insert(add.clone(), vec!['O', 'O']);

        let body = FnBodyRC::Let(z.clone(), ExprRC::Pap(add_wrap, vec![x.clone()]), retour.clone());
        let expected = HashSet::from( [x.clone()]);
        assert_eq!(expected, collect_o(body, beta));
        
    }  

    /*cas où z appartient à collect_O(F) */
    #[test]
    fn  test_proj_1() {
        let var = Var::Var(String::from("var"));
        let x = Var::Var(String::from("x"));
        let z = Var::Var(String::from("z")); 
        let retour = Box::new(FnBodyRC::Ret(z.clone()));

        let body = FnBodyRC::Let(x.clone(), ExprRC::Proj(0, var.clone()), 
            Box::new(FnBodyRC::Let(z.clone(),ExprRC::Reset(x.clone()),retour.clone())));
        let expected = HashSet::from( [x.clone(), var.clone()]);
        assert_eq!(expected, collect_o(body, HashMap::new()));
    }   

    /*cas où z n'appartient pas à collect_O(F) */
    #[test]
    fn  test_proj_2() {
        let x = Var::Var(String::from("x"));
        let z = Var::Var(String::from("z")); 
        let retour = Box::new(FnBodyRC::Ret(z.clone()));
        let body = FnBodyRC::Let(z, ExprRC::Proj(0, x), retour);
        let expected: HashSet<Var> = HashSet::new();
        assert_eq!(expected, collect_o(body, HashMap::new()))

    }  

    #[test]
    fn test_ret() {
        let body = FnBodyRC::Ret(Var::Var(String::from("x")));
        let expected: HashSet<Var> = HashSet::new();
        assert_eq!(expected, collect_o(body, HashMap::new()))
    }

    #[test]
    fn test_case() {
        let xs = Var::Var(String::from("xs"));
        let h1 = Var::Var(String::from("h1"));
        let retour = Box::new(FnBodyRC::Ret(h1.clone()));
        let case1 = FnBodyRC::Ret(xs.clone());
        let case2 = FnBodyRC::Let(h1.clone(), ExprRC::Proj(1, xs.clone()), retour);

        let body = FnBodyRC::Case(xs.clone(), vec![case1,case2]);
        let expected: HashSet<Var> = HashSet::new();
        assert_eq!(expected, collect_o(body, HashMap::new()));
    }

}

#[cfg(test)]
mod tests_inferring {
    use crate::reader;
    use std::fs;
    use chumsky::Parser;

    fn id_pair() {
        let file_path = "./examples/id_pair.pstl";
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("unable to read file + {}", file_path).as_str());
        let prog = reader::program().parse(file_contents).expect("can't parse");
    }   
}