use crate::ast::Var;
use crate::ast::Const;
use std::collections::HashMap;

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
    Pap(Const, Vec<Var>),
    Ctor(i32, Vec<Var>),
    Proj(i32, Var),
    Num(i32),
    Reset(Var),
    Reuse(Var, i32, Vec<Var>),
}

#[derive(Debug, Clone)]
pub enum FnRC {
    Fn(Vec<Var>, FnBodyRC),
}

#[derive(Debug, Clone)]
pub enum ProgramRC {
    Program(HashMap<Const,FnRC>),
}