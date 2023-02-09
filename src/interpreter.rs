mod ast;
use ast::Var;
use ast::Expr;
use ast::FnBody;
use ast::Fn;
use ast::AST;



fn eval

fn eval_expr (expr:Expr) -> Expr {
    match expr {
        FnCall(ident, vars) => ,
        Pap(ident, vars) => ,
        Ctor(n, vars) => ,
        Proj(n, vars) => ,
        Value(n) =>,
    }
}

fn eval_ast (ast:AST) -> Expr {
    match ast  {
        Var(var) => eval_var(var),
        Expr(expr) => eval_expr(expr),
        FnBody(body) => eval_fnbody(body),
    }
}