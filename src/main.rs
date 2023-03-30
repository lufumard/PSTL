#![allow(dead_code)]

mod ast;
use chumsky::Parser;
use interpreter::Heap;
use interpreter::Value;
use interpreter::Ctxt;
use interpreter::Var;
use std::fs;


//use crate::interpreter::{start_interpreter, empty_heap, empty_ctxt};

pub mod reader;

#[path = "interpreter/primitives.rs"]
mod primitives;

#[path = "interpreter/interpreter.rs"]
mod interpreter;

#[path = "interpreter/tests.rs"]
mod tests_interpreter;

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

}



