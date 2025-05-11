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
use std::{cell::RefCell, fs, process::ExitCode, rc::Rc};

fn main() -> ExitCode {
    let tests = match fs::read_dir("tests") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error: Failed to read the tests directory: {e}");
            return ExitCode::FAILURE;
        }
    };

    for test in tests {
        let path = match test {
            Ok(entry) => entry.path(),
            Err(e) => {
                eprintln!("Error while instantiating test: {e}");
                return ExitCode::FAILURE;
            }
        };
        if path.is_file() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    println!("Running test: {}", path.display());
                    run(&content);
                }
                Err(e) => {
                    eprintln!("Failed to load test {}: {e}", path.display());
                }
            }
        }
    }

    ExitCode::SUCCESS
}

fn run(source: &str) {
    let mut lexer = Lexer::new();
    let tokens = match lexer.lex(source) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {e}");
            return;
        }
    };

    let mut parser = Parser::new();
    let ast = match parser.parse(tokens) {
        Ok(ast) => ast,
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
