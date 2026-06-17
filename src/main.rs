mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;

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
            println!("parsed successfully.");
            println!("{:#?}", program);
        }
        Err(e) => {
            println!("parse error: {}", e);
        }
    }
}
