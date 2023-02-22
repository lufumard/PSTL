#![allow(dead_code)]

mod ast;
use chumsky::Parser;
use interpreter::Heap;
use interpreter::Value;
use interpreter::Ctxt;
use interpreter::Var;
use std::fs;

use crate::interpreter::{start_interpreter, empty_heap, empty_ctxt};

pub mod reader;

#[path = "interpreter/primitives.rs"]
mod primitives;

#[path = "interpreter/interpreter.rs"]
mod interpreter;

fn add_value(var:Var, v:Value, c:Ctxt, h:&mut Heap) -> Ctxt {
    c.add(var, h.add((v, 1)))
}

fn main() {
    let file_path = "./examples/swap.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::ast().parse(file_contents).expect("can't parse");
    drop(parsed);
    //dbg!(parsed);


    let file_path = "./examples/fibo.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::ast().parse(file_contents).expect("can't parse");
    let n = 7;
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), Value::Num(n), 
        add_value(Var::Var("m1".to_string()), Value::Num(1), empty_ctxt(), &mut heap), &mut heap);
 
    let call = ast::Expr::FnCall(ast::Const::Const("fibo".to_string()), vec![ast::Var::Var("n".to_string()), ast::Var::Var("m1".to_string())]);
    let res = start_interpreter(vec![parsed], call, &ctxt, &mut heap);
    println!("fibo {} =", n);
    dbg!(heap.get(res));

    // fibo 7  = 21
    // fibo 10 = 89
    
    let file_path = "./examples/pap.pstl";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("unable to read file + {}", file_path).as_str());
    let parsed = reader::ast().parse(file_contents).expect("can't parse");
    let mut heap = empty_heap();
    let ctxt = add_value(Var::Var("n".to_string()), Value::Num(10), 
                add_value(Var::Var("m".to_string()), Value::Num(6), empty_ctxt(), &mut heap)
            , &mut heap);
 
    let call = ast::Expr::FnCall(ast::Const::Const("pap".to_string()), vec![ast::Var::Var("n".to_string()), ast::Var::Var("m".to_string())]);
    let res = start_interpreter(vec![parsed], call, &ctxt, &mut heap);
    println!("pap 10 6 = 4");
    dbg!(heap.get(res));

}
