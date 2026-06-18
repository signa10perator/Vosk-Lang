mod lexer;
mod parser;
mod ast;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let source = "
~ anomaly {
    src :: ?
    freq :: %
    signal :: +
    ! signal :: +
    @ freq :: x
}
";

    let lexer = Lexer::new(source);
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
