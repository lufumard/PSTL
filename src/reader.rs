use crate::ast::{Const, Expr, Fn, FnBody, Var, AST};
use chumsky::{
    prelude::*,
    text::{ident, int, keyword},
    Parser,
};

pub(crate) fn var() -> impl Parser<char, Var, Error = Simple<char>> {
    ident().padded().map(Var::Var)
}

pub(crate) fn const_() -> impl Parser<char, Const, Error = Simple<char>> {
    ident().padded().map(Const::Const)
}

pub(crate) fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    let fncall = const_()
        .padded()
        .then(var().repeated())
        .map(|(_fn_name, _args)| Expr::FnCall(_fn_name, _args));

    let pap = keyword("proj")
        .ignore_then(int(10))
        .padded()
        .then(var().padded())
        .map(|(_int, _var)| {
            let i = _int.parse().expect("can't parse int in proj");
            Expr::Proj(i, _var)
        });

    let partial_fn_call = var()
        .padded()
        .then(var().padded())
        .map(|(_x, _y)| Expr::PartialFnCall(_x, _y));

    let ctor = keyword("ctor")
        .ignore_then(int(10))
        .padded()
        .then(var().repeated())
        .map(|(_int, _vars)| {
            let i = _int.parse().expect("not an int in ctor");
            Expr::Ctor(i, _vars)
        });

    let proj = keyword("proj")
        .ignore_then(int(10))
        .padded()
        .then(var().padded())
        .map(|(_int, _var)| {
            let i = _int.parse().expect("not an int in proj");
            Expr::Proj(i, _var)
        });

    fncall.or(pap).or(partial_fn_call).or(ctor).or(proj)
}

pub(crate) fn fnbody() -> impl Parser<char, FnBody, Error = Simple<char>> {
    recursive(|fnbody_rec| {
        let ret = keyword("ret")
            .padded()
            .ignore_then(var())
            .padded()
            .map(|_ident| FnBody::Ret(_ident));

        let let_ = keyword("let")
            .padded()
            .ignore_then(var())
            .then_ignore(just('=').padded())
            .then(expr().padded())
            .then_ignore(just(';').padded())
            .then(fnbody_rec.clone().padded())
            .padded()
            .map(|((_var, _expr), _fnbody)| FnBody::Let(_var, _expr, Box::new(_fnbody)));

        let case = keyword("case")
            .padded()
            .ignore_then(var())
            .then_ignore(keyword("of").padded())
            .then(
                fnbody_rec
                    .padded()
                    .delimited_by(just('('), just(')'))
                    .padded()
                    .repeated(),
            )
            .padded()
            .map(|(_var, _fnbodies)| FnBody::Case(_var, _fnbodies));

        ret.or(let_).or(case)
    })
}

pub(crate) fn fun() -> impl Parser<char, Fn, Error = Simple<char>> {
    const_()
        .then(var().repeated())
        .then_ignore(just('=').padded())
        .then(fnbody())
        .map(|((_const, _args), _fnbody)| Fn::Fn(_const, _args, _fnbody))
}

pub(crate) fn ast() -> impl Parser<char, AST, Error = Simple<char>> {
    fun()
        .padded()
        .map(AST::Fn)
        .or(fnbody().padded().map(AST::FnBody))
        .or(expr().padded().map(AST::Expr))
        .or(var().padded().map(AST::Var))
        .then_ignore(end())
}
