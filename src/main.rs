use lexer::lexer::Lexer;

mod error;
mod lexer;

fn main() {
    let source = "fn 123 Hello";
    dbg!(source);
    let mut lexer = Lexer::new(source); 
    match lexer.lex() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
}
