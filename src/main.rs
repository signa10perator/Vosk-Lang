mod lexer;
mod parser;
mod ast;

use lexer::{Lexer, Token};

fn main() {
    let source = "~ anomaly { src :: ? freq :: % }";
    let mut lex = Lexer::new(source);

    loop {
        let tok = lex.next_token();
        println!("{:?}", tok);
        if tok == Token::EOF {
            break;
        }
    }
}
