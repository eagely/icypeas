use lexer::lexer::Lexer;

mod error;
mod lexer;

fn main() {
    let source = ":(){:|:&};:";
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
