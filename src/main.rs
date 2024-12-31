mod error;
mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let source = "x : int int int 3 + 3 x a b = (n t $ n + t) if true false elif false true else (x t $ x - t)";
    dbg!(source);
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };
    
    let mut parser = Parser::new(tokens);
    dbg!(parser.parse().unwrap());
}
