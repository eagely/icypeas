#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_inception)]
#![macro_use]
mod error;
mod interpreter;
mod lexer;
mod model;
mod parser;

use error::Result;
use interpreter::{environment::Environment, interpreter::Interpreter};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{
    cell::RefCell,
    fs::{self, read_dir},
    process::ExitCode,
    rc::Rc,
};

fn main() -> ExitCode {
    test()
}

fn test() -> ExitCode {
    let tests = match read_dir("tests") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error: Failed to read the tests directory: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut failed_tests = Vec::new();

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
                Ok(content) => match run(&content) {
                    Ok(()) => println!(
                        "\x1b[32mSUCCESS\x1b[0m {} completed successfully.",
                        path.display()
                    ),
                    Err(e) => {
                        eprintln!(
                            "\x1b[31mFAILED\x1b[0m {} failed with error: {e}",
                            path.display()
                        );
                        failed_tests.push(path.display().to_string());
                    }
                },
                Err(e) => {
                    eprintln!("Failed to load test {}: {e}", path.display());
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    if !failed_tests.is_empty() {
        eprintln!("The following tests failed:");
        for test in failed_tests {
            eprintln!("- {test}");
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run(source: &str) -> Result<()> {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(source)?;

    let mut parser = Parser::new();
    let ast = parser.parse(tokens)?;

    let environment = Rc::new(RefCell::new(Environment::new()));
    let mut interpreter = Interpreter::new(environment);
    interpreter.interpret(ast)?;

    Ok(())
}
