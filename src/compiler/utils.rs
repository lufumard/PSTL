use super::{Var, Const,ast_rc::{ExprRC, FnBodyRC, ConstWrapper, Either}};

/*Verifie si la variable z est dans l'expression e
si z in e , true, sinon false
*/
pub fn is_in_expr(z:Var, e: ExprRC) -> bool {
    match e {
        ExprRC::FnCall(_, vars) =>  vars.into_iter().any(|x| x == z),
        ExprRC::Pap(_, vars) => vars.into_iter().any(|x| x == z),
        ExprRC::Ctor(_, vars) => vars.into_iter().any(|x| x == z),
        ExprRC::Proj(_, var) => var == z,
        ExprRC::Num(_) => false,
        ExprRC::PapCall(ident, var) => ident == var || var == z,
        ExprRC::Reset(var) => var == z,
        ExprRC::Reuse(var, _, vars) => (var == z) || match vars {
                Either::Left(_) => false,
                Either::Right(vs) => vs.into_iter().any(|x| x == z),
            },
    }
    
}

pub fn is_in_fn(z: Var, f:FnBodyRC) -> bool {
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

pub fn wrap_const(c : Const) -> ConstWrapper {
    let Const::Const(mut name) = c.clone();
    name.push_str("_c");
    ConstWrapper::ConstWrapper(Const::Const(name), c)
}