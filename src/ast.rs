#[derive(Debug)]
enum Var {
    Nom(String),
}

#[derive(Debug)]
enum Expr {
    FnCall(Const, Vec<Var>),
    Pap(Const, Vec<Var>),
    Ctor(i32, Vec<Var>),
    Proj(i32, Vec<Var>),
}

#[derive(Debug)]
enum FnBody {
    Ret(Var),
    Let(Var, Expr, Box<FnBody>),
    Case(Var, Vec<Box<FnBody>>),
}

#[derive(Debug)]
enum Fn {
    Fn(Vec<Var>, FnBody),
}

#[derive(Debug)]
enum Const {
    Ident(String),
}

#[derive(Debug)]
enum AST {
    Const(Const),
    Var(Var),
    Expr(Expr),
    FnBody(FnBody),
}
