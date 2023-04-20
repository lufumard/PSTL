use crate::ast::{Const, Expr, Fn, FnBody, Var, Program};
use indexmap::IndexMap;

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
    let num = text::int::<char, Simple<char>>(10)
        .map(|s| Expr::Num(s.parse().expect("can't parse int")))
        .padded();

    let fncall = const_()
        .padded()
        .then(var().repeated())
        .map(|(_fn_name, _args)| Expr::FnCall(_fn_name, _args));

    let pap = keyword("pap")
        .padded()
        .ignore_then(const_())
        .padded()
        .then(var().repeated())
        .map(|(_const, _vars)| Expr::Pap(_const, _vars));

    let ctor = just("ctor")
        .ignore_then(int(10))
        .padded()
        .then(var().repeated())
        .map(|(_int, _vars)| {
            let i = _int.parse().expect("not an int in ctor");
            Expr::Ctor(i, _vars)
        });

    let proj = just("proj")
        .ignore_then(int(10))
        .padded()
        .then(var().padded())
        .map(|(_i, _var)| {
            let i = _i.parse().expect("not an int in proj");
            Expr::Proj(i, _var)
        });

    pap.or(ctor).or(proj).or(fncall).or(num)
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
    var().repeated()
        .then_ignore(just('=').padded())
        .then(fnbody())
        .map(|(_args, _fnbody)| Fn::Fn(_args, _fnbody))
}


pub(crate) fn fn_dec() -> impl Parser<char, (Const, Fn), Error = Simple<char>> {
    const_()
        .then(fun())
        .map(|(_const, _fn)| (_const, _fn))
}

pub(crate) fn program() -> impl Parser<char, Program, Error = Simple<char>> {
    fn_dec()
        .padded()
        .repeated()
        .padded()
        .then_ignore(end())
        .map(|f| Program::Program(f.into_iter()
        .map(|(_const, _fn)| (_const, _fn))
        .collect::<IndexMap<Const, Fn>>()))
}
