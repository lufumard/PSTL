use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq,Eq,Hash)]
pub enum Var {
    Var(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Const {
    Const(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    FnCall(Const, Vec<Var>),
    PapCall(Var, Var),
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
    Fn(Vec<Var>, FnBody),
}


#[derive(Debug, Clone)]
pub enum Program {
    Program(IndexMap<Const, Fn>),
}



pub const CONST_FALSE : i32 = 0;
pub const CONST_TRUE : i32 = 1;
pub const CONST_NIL : i32 = 2;
pub const CONST_LIST : i32 = 3;
pub const CONST_NUM : i32 = 4;
