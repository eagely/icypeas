use super::enums::{Location, Token, TokenKind};

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

    fn emit(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            location: Location {
                row: self.row,
                column: self.index - self.bol,
            },
        })
    }

    fn next(&mut self) -> Option<&char> {
        self.index += 1;
        self.source.get(self.index)
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.index + 1)
    }
}
