mod ast;
use chumsky::Parser;
use std::fs;

use crate::interpreter::{start_interpreter, empty_heap};

pub mod reader;

#[path = "interpreter/primitives.rs"]
mod primitives;

#[path = "interpreter/interpreter.rs"]
mod interpreter;

fn main() {
    let file_path = "./examples/swap.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::ast().parse(file_contents).expect("can't parse");
    //dbg!(parsed);


    let file_path = "./examples/fibo.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::ast().parse(file_contents).expect("can't parse");
    let n = 7;
    let heap = empty_heap()
        .add(ast::Var::Var("n".to_string()), ast::Expr::Num(n))
        .add(ast::Var::Var("m1".to_string()), ast::Expr::Num(1));   
    let call = ast::Expr::FnCall(ast::Const::Const("fibo".to_string()), vec![ast::Var::Var("n".to_string()), ast::Var::Var("m1".to_string())]);
    let res = start_interpreter(vec![parsed], call, heap);
    println!("fibo {} =", n);
    dbg!(res);

    // fibo 7  = 21
    // fibo 10 = 89

    let file_path = "./examples/pap.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::ast().parse(file_contents).expect("can't parse");
    let heap = empty_heap()
        .add(ast::Var::Var("n".to_string()), ast::Expr::Num(10))
        .add(ast::Var::Var("m".to_string()), ast::Expr::Num(6));   
    let call = ast::Expr::FnCall(ast::Const::Const("pap".to_string()), vec![ast::Var::Var("n".to_string()), ast::Var::Var("m".to_string())]);
    let res = start_interpreter(vec![parsed], call, heap);
    dbg!("pap 10 6 = 4", res);
}
