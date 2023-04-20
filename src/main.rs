#![allow(dead_code)]

mod ast;
use chumsky::Parser;
use std::env;
use std::fs;

//use crate::interpreter::{start_interpreter, empty_heap, empty_ctxt};

pub mod reader;

#[path = "interpreter/interpreter.rs"]
mod interpreter;

#[path = "compiler/compiler.rs"]
mod compiler;

#[path = "interpreter/tests.rs"]
#[cfg(test)]
mod tests_interpreter;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage : pstl <-c|-i> <file.pstl>")
    }
    match args.get(2) {
        Some(file_path) => {
            let file_contents = fs::read_to_string(file_path)
                .expect(format!("unable to read file + {}", file_path).as_str());
            let parsed = reader::ast().parse(file_contents).expect("can't parse");
            match args.get(3) {
                Some(out) => {
                    match args.get(1) {
                        Some(e) => {
                            if e.to_owned() == "-i".to_string() {
                                interpreter::interpreter(parsed, out);
                            } else if  e.to_owned() == "-c".to_string() {
                                //compiler::compile(parsed, out);
                            }
                        },
                        None => panic!("Il manque le nom de sortie ou la fonction à exécuter"),
                    }
                },
                None => panic!("Pas assez d'arguments"),
            }
        },
        None => panic!("Pas assez d'arguments"),
    }
}



