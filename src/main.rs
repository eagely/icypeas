#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_inception)]
#![macro_use]
mod error;
mod interpreter;
mod lexer;
mod model;
mod parser;

use interpreter::{environment::Environment, interpreter::Interpreter};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let source = r"
        churchTrue f s = f
        churchTrue 1 2

        churchFalse f s = s
        churchFalse 1 2

        add a b = a + b
        add 4 7

        addTwice f s = f + s + s
        addTwice 1 10

        (y $ (x $ y)) 1 2
        (y $ (x $ x)) 1 2
        (x $ x+2) 1

        1 == 1
        3 + 4
        10 - 5
        2 * 3
        20 / 4
        20 % 3
        2 ** 3
        5 & 3
        5 | 3
        5 ^ 3
        7 == 7
        7 != 8
        3 < 10
        10 <= 10
        15 > 5
        15 >= 15

        !false
        !true
        -7
        -(-7)
       
        if false then 1 else 2
        if true then 2 else 1
        if false then 1 elif false then 3 else 2
        if (2 + 1) == 3 then 2 else 1
        if 2 + 1 == 3 then 2 else 1
        if 2 + 1 != 3 then 1 else 2
        
        twoIfOneElseOne x = if x == 1 then 2 else 1
        twoIfOneElseOne 1
        twoIfOneElseOne 17
        
        add 2 (if false then 1 else 2)
    ";
    println!("Source: {source}");

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.lex() {
        Ok(tokens) => {
            dbg!(&tokens);
            tokens
        }
        Err(e) => {
            eprintln!("Lexer error: {e}");
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
            eprintln!("Parser error: {e}");
            return;
        }
    };

    let environment = Rc::new(RefCell::new(Environment::new()));
    for expr in ast {
        let mut interpreter = Interpreter::new(environment.clone());
        match interpreter.interpret(expr) {
            Ok(result) => println!("{result}"),
            Err(e) => {
                eprintln!("Interpreter error: {e}");
                return;
            }
        }
    }
}
