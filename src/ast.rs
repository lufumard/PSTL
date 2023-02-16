#[derive(Debug, Clone)]
pub enum Var {
    Var(String),
}

#[derive(Debug, Clone)]
pub enum Const {
    Const(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    FnCall(Const, Vec<Var>),
    PartialFnCall(Var, Var),
    Pap(Const, Vec<Var>),
    Ctor(i32, Vec<Var>),
    Proj(i32, Var),
    Num(i32),
}

#[derive(Debug, Clone)]
pub enum FnBody {
    Ret(Var),
    Let(Var, Expr, Box<FnBody>),
    Case(Var, Vec<FnBody>),
}

#[derive(Debug, Clone)]
pub enum Fn {
    Fn(Const, Vec<Var>, FnBody),
}

#[derive(Debug, Clone)]
pub enum AST {
    Const(Const),
    Var(Var),
    Expr(Expr),
    FnBody(FnBody),
    Fn(Fn),
    Program(Const, Fn)
}


pub const CONST_FALSE : i32 = 0;
pub const CONST_TRUE : i32 = 1;
pub const CONST_NIL : i32 = 2;
pub const CONST_LIST : i32 = 3;
