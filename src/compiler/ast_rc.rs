use crate::ast::{Var, Const};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstWrapper {
    //(novelle constante, constate qu'on wrappe)
    ConstWrapper(Const, Const)
}
#[derive(Debug, Clone, PartialEq)]
pub enum FnBodyRC {
    Ret(Var),
    Let(Var, ExprRC, Box<FnBodyRC>),
    Case(Var, Vec<FnBodyRC>),
    Inc(Var, Box<FnBodyRC>),
    Dec(Var, Box<FnBodyRC>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprRC{
    FnCall(Const, Vec<Var>),
    PapCall(Var, Var),
    Pap(ConstWrapper, Vec<Var>),
    Ctor(i32, Vec<Var>),
    Proj(i32, Var),
    Num(i32),
    Reset(Var),
    Reuse(Var, i32, Vec<Var>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnRC {
    Fn(Vec<Var>, FnBodyRC),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramRC {
    Program(IndexMap<Const, FnRC>),
}