#![allow(dead_code)]
use crate::ast::FnBody;
use crate::ast::Var;
use crate::ast::Expr;
use crate::ast::Fn;
use crate::ast_rc::ExprRC;
use crate::ast_rc::FnBodyRC;
use crate::ast_rc::FnRC;
use crate::interpreter::get_nb_args_ctor;

/*Verifie si la variable z est dans l'expression e
si z in e , true, sinon false
*/
fn is_in_expr(z:Var, e: ExprRC) -> bool {
    match e {
        ExprRC::FnCall(_, vars) =>  vars.into_iter().any(|x| x == z),
        ExprRC::Pap(_, vars) => vars.into_iter().any(|x| x == z),
        ExprRC::Ctor(_, vars) => vars.into_iter().any(|x| x == z),
        ExprRC::Proj(_, var) => var == z,
        ExprRC::Num(_) => false,
        ExprRC::PapCall(ident, var) => ident == var || var == z,
        ExprRC::Reset(var) => var == z,
        ExprRC::Reuse(var, _, vars) => (var == z) || vars.into_iter().any(|x| x == z),
    }
    
}

fn is_in_fn(z: Var, f:FnBodyRC) -> bool {
    match f {
        FnBodyRC::Ret(var) => var == z,
        FnBodyRC::Let(var, expr, fnbody) => {
            var == z || is_in_expr(z.clone(),expr) || is_in_fn(z.clone(), *fnbody)
        },
        FnBodyRC::Case(var, bodys) => var == z || bodys.into_iter().any(|x| is_in_fn(z.clone(),x)),
        FnBodyRC::Inc(var, fnbody) => var == z || is_in_fn(z, *fnbody),
        FnBodyRC::Dec(var, fnbody) => var == z || is_in_fn(z, *fnbody),
    }
}

fn expr_pure_rc(e:Expr) -> ExprRC {
    match e {
        Expr::FnCall(ident, vars) => ExprRC::FnCall(ident, vars),
        Expr::PapCall(ident, var) => ExprRC::PapCall(ident, var),
        Expr::Pap(ident, vars) => ExprRC::Pap(ident, vars),
        Expr::Ctor(n, vars) => ExprRC::Ctor(n, vars),
        Expr::Proj(n, var) => ExprRC::Proj(n, var),
        Expr::Num(n) => ExprRC::Num(n),
    }
}

pub fn get_nb_args_ctor_usize(n: usize) -> usize {
    get_nb_args_ctor(n as i32) as usize
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

pub fn delta_reuse(c : Fn) -> FnRC {
    match c {
        Fn::Fn(vars, fnbody) => FnRC::Fn(vars, R(fnbody)),
    }
}

#[allow(non_snake_case)]
pub fn R(body: FnBody) -> FnBodyRC {
    let mut w = W::new(String::from("w"), 0);
    match body {
        FnBody::Ret(x) => FnBodyRC::Ret(x),
        FnBody::Let(x, e, fnbody) => FnBodyRC::Let(x,expr_pure_rc(e), Box::new(R(*fnbody))),
        FnBody::Case(x, bodys) => {
            FnBodyRC::Case(x.clone(), ((*bodys.iter().enumerate().map(|(i, fi)| 
                D(x.clone(), get_nb_args_ctor_usize(i), R(fi.clone()), & mut w)).collect::<Vec<FnBodyRC>>())).to_vec())
        } ,
    }
}

#[allow(non_snake_case)]
pub fn D(z:Var, n:usize,body:FnBodyRC, w: &mut W) -> FnBodyRC {
    let b = &body;
    match body {
        FnBodyRC::Ret(_) => body,
        FnBodyRC::Case(x, bodys) => {
            FnBodyRC::Case(x, bodys.into_iter().map(|f| D(z.clone(),n,f, w)).collect())
        },
        FnBodyRC::Let(ref x, ref e, ref fnbody) => {
            match is_in_expr(z.clone(), e.clone()) || is_in_fn(z.clone(), *fnbody.clone()) {
                true => FnBodyRC::Let(x.clone(), e.clone(), Box::new(D(z,n,*fnbody.clone(), w))),
                false => body,
            }
        }
        _ => {
            let temp = S(w.val(),n,b.clone());
            match temp.clone() != body.clone() {
                    true => {
                        FnBodyRC::Let(w.get_and_inc(),ExprRC::Reset(z.clone()), Box::new(temp.clone()))
                    },
                    false => body,
                }
        } ,
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
            _ => S(w, n, *fnbody),
        },
        FnBodyRC::Inc(_, fnbody) => S(w,n, *fnbody),
        FnBodyRC::Dec(_, fnbody) => S(w,n, *fnbody),
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::ast_rc::ExprRC;
    use crate::ast_rc::FnBodyRC;
    use crate::ast::Var;
    use crate::test_compiler_rc::D;
    use crate::test_compiler_rc::S;
    use crate::test_compiler_rc::W;

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
    fn test_case_1() {
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
    fn test_case_2() {
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


    
}
