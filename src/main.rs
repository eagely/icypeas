mod error;
mod interpreter;
mod lexer;
mod parser;

use interpreter::{environment::Environment, interpreter::Interpreter};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let source = r#"if false "then" else "else"; if true "then" else "else""#;
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

    let environment = Rc::new(RefCell::new(Environment::new()));
    for expr in ast {
        let interpreter = Interpreter::new(expr, Rc::clone(&environment));
        match interpreter.interpret() {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Interpreter error: {}", e);
                return;
            }
        }
    }
}
