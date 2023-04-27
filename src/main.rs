#![allow(dead_code)]

mod ast;
use chumsky::Parser;
use transform_var::transform_program;
use std::env;
use std::fs;
use std::fs::File;

//use crate::interpreter::{start_interpreter, empty_heap, empty_ctxt};

pub mod reader;

#[path = "interpreter/interpreter.rs"]
mod interpreter;

#[path = "compiler/compiler.rs"]
mod compiler;

#[path = "interpreter/tests.rs"]
#[cfg(test)]
mod tests_interpreter;

mod transform_var;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage : pstl <c|i> <file.pstl> <output_file.wat|fun_to_run>")
    }
    match args.get(2) {
        Some(file_path) => {
            match args.get(3) {
                Some(out) => {
                    match args.get(1) {
                        Some(e) => {
                            let file_contents = fs::read_to_string(file_path)
                                .expect(format!("unable to read file + {}", file_path).as_str());
                            let parsed = reader::program().parse(file_contents).expect("can't parse");
                            let program = transform_program(parsed);
                            if e.to_owned() == "i".to_string() {
                                interpreter::interpreter(program, out);
                            } else if  e.to_owned() == "c".to_string() {
                                let mut file = File::create(out)
                                    .expect(&format!("Impossible d'ouvrir le fichier {out}"));
                                compiler::compile(program, &mut file);
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



