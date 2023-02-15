#[derive(Debug, Clone)]
pub(crate) enum Var {
    Var(String),
}

#[derive(Debug, Clone)]
pub(crate) enum Const {
    Const(String),
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    FnCall(Const, Vec<Var>),
    PartialFnCall(Var, Var),
    Pap(Const, Vec<Var>),
    Ctor(i32, Vec<Var>),
    Proj(i32, Var),
    Num(i32),
}

#[derive(Debug, Clone)]
pub(crate) enum FnBody {
    Ret(Var),
    Let(Var, Expr, Box<FnBody>),
    Case(Var, Vec<FnBody>),
}

#[derive(Debug, Clone)]
pub(crate) enum Fn {
    Fn(Const, Vec<Var>, FnBody),
}

#[derive(Debug, Clone)]
pub(crate) enum AST {
    Const(Const),
    Var(Var),
    Expr(Expr),
    FnBody(FnBody),
    Fn(Fn),
    Program(Const, Fn)
}


pub(crate) const CONST_FALSE : i32 = 0;
pub(crate) const CONST_TRUE : i32 = 1;
pub(crate) const CONST_NIL : i32 = 2;
pub(crate) const CONST_LIST : i32 = 3;
