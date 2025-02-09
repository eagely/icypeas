mod error;
mod interpreter;
mod lexer;
mod parser;

use interpreter::interpreter::Interpreter;
use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let source = "x : int int int 3.564564 + 3 x a b = (n t $ n + t) if true == 3 false elif false true else (x t $ x - t)";
    println!("Source: {}", source);

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.lex() {
        Ok(tokens) => {
            dbg!(&tokens);
            tokens
        }
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => {
            dbg!(&ast);
            ast
        }
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return;
        }
    };

    // for expr in ast {
    // let interpreter = Interpreter::new(expr);
    // match interpreter.interpret() {
    // Ok(result) => println!("{}", result),
    // Err(e) => {
    // eprintln!("Interpreter error: {}", e);
    // return;
    // }
    // }
    // }
}
