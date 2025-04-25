mod error;
mod interpreter;
mod lexer;
mod parser;

use interpreter::{environment::Environment, interpreter::Interpreter};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let source = r#"
        selectFirst s = f
        churchTrue f = selectFirst
        
        selectSecond s = s
        churchFalse f = selectSecond
        
        churchTrue 1 2
        churchFalse 1 2
    "#;
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
        let mut interpreter = Interpreter::new(Rc::clone(&environment));
        match interpreter.interpret(expr) {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Interpreter error: {}", e);
                return;
            }
        }
    }
}
