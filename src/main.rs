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
        add 2 2
        add "hello" " world"
        sub 4 3
        mul 10 12
        div 42 7
        modulo 5 2
        print "hello"

        add 170141183460469231731687303715884105727 1
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
        match interpreter.interpret(&expr) {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Interpreter error: {}", e);
                return;
            }
        }
    }
}
