use crate::compiler::ast_rc::{ExprRC, FnRC, FnBodyRC, ProgramRC};
use crate::compiler::ast_compiler::{Const, Var};
use indexmap::IndexMap;

use chumsky::{
    prelude::*,
    text::{ident, int, keyword},
    Parser,
};

use super::utils::wrap_const;

pub(crate) fn var() -> impl Parser<char, Var, Error = Simple<char>> {
    ident().padded().map(Var::Var)
}

pub(crate) fn const_() -> impl Parser<char, Const, Error = Simple<char>> {
    ident().padded().map(Const::Const)
}

pub(crate) fn expr() -> impl Parser<char, ExprRC, Error = Simple<char>> {
    let num = text::int::<char, Simple<char>>(10)
        .map(|s| ExprRC::Num(s.parse().expect("can't parse int")))
        .padded();

    let fncall = const_()
        .padded()
        .then(var().repeated())
        .map(|(_fn_name, _args)| ExprRC::FnCall(_fn_name, _args));

    let pap = keyword("pap")
        .padded()
        .ignore_then(const_())
        .padded()
        .then(var().repeated())
        .map(|(_const, _vars)| ExprRC::Pap(wrap_const(_const), _vars));

    let ctor = just("ctor")
        .ignore_then(int(10))
        .padded()
        .then(var().repeated())
        .map(|(_int, _vars)| {
            let i = _int.parse().expect("not an int in ctor");
            ExprRC::Ctor(i, _vars)
        });

    let proj = just("proj")
        .ignore_then(int(10))
        .padded()
        .then(var().padded())
        .map(|(_i, _var)| {
            let i = _i.parse().expect("not an int in proj");
            ExprRC::Proj(i, _var)
        });
    
    let reset = keyword("reset")
        .padded()
        .ignore_then(var())
        .map(|x| ExprRC::Reset(x));
    
    let reuse = keyword("reuse")
    .padded()
    .ignore_then(var()).padded()
    .then_ignore(just("in ctor"))
    .then(int(10)).padded()
    .then(var().repeated())
    .padded()//.then_ignore(just(')'))
    .map(|((x, ident), vars)| {
        let i = ident.parse().expect("not an int in reuse");
        ExprRC::Reuse(x, i, vars)
    });

    pap.or(ctor).or(proj).or(reuse).or(reset).or(fncall).or(num)
}

pub(crate) fn fnbody() -> impl Parser<char, FnBodyRC, Error = Simple<char>> {
    recursive(|fnbody_rec| {
        let ret = keyword("ret")
            .padded()
            .ignore_then(var())
            .padded()
            .map(|_ident| FnBodyRC::Ret(_ident));

        let let_ = keyword("let")
            .padded()
            .ignore_then(var())
            .then_ignore(just('=').padded())
            .then(expr().padded())
            .then_ignore(just(';').padded())
            .then(fnbody_rec.clone().padded())
            .padded()
            .map(|((_var, _expr), _fnbody)| FnBodyRC::Let(_var, _expr, Box::new(_fnbody)));

        let case = keyword("case")
            .padded()
            .ignore_then(var())
            .then_ignore(keyword("of").padded())
            .then(
                fnbody_rec.clone()
                    .padded()
                    .delimited_by(just('('), just(')'))
                    .padded()
                    .repeated(),
            )
            .padded()
            .map(|(_var, _fnbodies)| FnBodyRC::Case(_var, _fnbodies));
        
        let inc= keyword("inc")
        .padded()
        .ignore_then(var())
        .then_ignore(just(';').padded())
        .then(fnbody_rec.clone().padded())
        .padded()
        .map(|(var, fnbody)| FnBodyRC::Inc(var, Box::new(fnbody)));

        let dec= keyword("dec")
        .padded()
        .ignore_then(var())
        .then_ignore(just(';').padded())
        .then(fnbody_rec.clone().padded())
        .padded()
        .map(|(var, fnbody)| FnBodyRC::Dec(var, Box::new(fnbody)));

        ret.or(let_).or(case).or(inc).or(dec)
    })
}

pub(crate) fn fun() -> impl Parser<char, FnRC, Error = Simple<char>> {
    var().repeated()
        .then_ignore(just('=').padded())
        .then(fnbody())
        .map(|(_args, _fnbody)| FnRC::Fn(_args, _fnbody))
}


pub(crate) fn fn_dec() -> impl Parser<char, (Const, FnRC), Error = Simple<char>> {
    const_()
        .then(fun())
        .map(|(_const, _fn)| (_const, _fn))
}

pub(crate) fn program() -> impl Parser<char, ProgramRC, Error = Simple<char>> {
    fn_dec()
        .padded()
        .repeated()
        .padded()
        .then_ignore(end())
        .map(|f| ProgramRC::Program(f.into_iter()
        .map(|(_const, _fn)| (_const, _fn))
        .collect::<IndexMap<Const, FnRC>>()))
}
