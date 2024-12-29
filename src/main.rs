mod error;
mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let source = ".1 1.0 -0 -1245 123.456 1";
    dbg!(source);
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };
    
    dbg!(&tokens);

    let mut parser = Parser::new(tokens);
}
