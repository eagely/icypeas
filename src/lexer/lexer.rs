use super::enums::Token;

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    index: usize,
    row: usize,
    bol: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            index: 0,
            row: 0,
            bol: 0,
        }
    }
}
