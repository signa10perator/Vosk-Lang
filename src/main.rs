mod lexer;
mod parser;
mod ast;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("VØSK v0.1.0");
        println!("usage: vosk run <file.vsk>");
        return;
    }

    let command = &args[1];
    let filepath = &args[2];

    match command.as_str() {
        "run" => {
            let source = match fs::read_to_string(filepath) {
                Ok(contents) => contents,
                Err(e) => {
                    println!("error: could not read '{}': {}", filepath, e);
                    return;
                }
            };

            let lexer = Lexer::new(&source);
            let mut parser = Parser::new(lexer);

            match parser.parse_program() {
                Ok(program) => {
                    let mut interpreter = Interpreter::new();
                    interpreter.run_program(&program);
                }
                Err(e) => {
                    println!("parse error: {}", e);
                }
            }
        }
        _ => {
            println!("unknown command: {}", command);
            println!("usage: vosk run <file.vsk>");
        }
    }
}
