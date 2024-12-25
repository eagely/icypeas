use super::enums::{Location, Token, TokenKind};
use crate::error::Result;

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

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        if let Some(c) = self.peek() {
            if c.is_alphabetic() || c == &'_' {
                self.consume_identifier();
            }
        }
        Ok(self.tokens.to_owned())
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

    fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.source.get(self.index).copied()
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.index + 1)
    }

    fn consume_identifier(&mut self) {
        let start = self.index;

        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != &'_' {
                break;
            }
            self.next();
        }

        let identifier: String = self.source[start..=self.index].iter().collect();

        let kind = match identifier.as_str() {
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "loop" => TokenKind::Loop,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => TokenKind::Identifier(identifier),
        };
        self.emit(kind);
    }
}
