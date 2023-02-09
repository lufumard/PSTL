#[path = "../ast.rs"]
mod ast;
use ast::Expr;
use ast::Expr;
use ast::FnBody;
use ast::Fn;
use ast::AST;

#[path = "../interpreter.rs"]
mod interpreter;
use interpreter::eval_expr;

// Définition des primitives
fn add_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);

    return eval_expr(vars[0]) + eval_expr(vars[1]);
}

fn sub_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) - eval_expr(vars[1]);
}

fn mul_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) * eval_expr(vars[1]);
}

fn sub_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) / eval_expr(vars[1]);
}

fn mod_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) % eval_expr(vars[1]);
}

fn and_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) && eval_expr(vars[1]);
}

fn or_fn(vars:Vec<Expr>) -> i32 {
    assert_eq!(vars.len(), 2);
    if(eval_expr(vars[0])== 0){
        if(eval_expr(vars[1]) == 0){
            return false;
        }
    }
    return 1;
}

fn sup_fn(vars:Vec<Expr>) -> bool {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) > eval_expr(vars[1]);
}

fn inf_fn(vars:Vec<Expr>) -> bool {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) < eval_expr(vars[1]);
}
fn supEq_fn(vars:Vec<Expr>) -> bool {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) >= eval_expr(vars[1]);
}

fn infEq_fn(vars:Vec<Expr>) -> bool {
    assert_eq!(vars.len(), 2);
    return eval_expr(vars[0]) <= eval_expr(vars[1]);
}