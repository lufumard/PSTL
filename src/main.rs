use chumsky::Parser;
use std::fs;

pub(crate) mod ast;
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
    dbg!(parsed);
}
