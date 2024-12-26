use super::enums::{Location, Token, TokenKind};
use crate::error::{Error, Result};

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
        while let Some(c) = self.current() {
            let kind = match c {
                c if c.is_whitespace() => {
                    self.advance();
                    continue;
                }
                _ => {
                    if c.is_digit(10) {
                        self.consume_number()?
                    } else if c.is_alphabetic() || c == '_' {
                        self.consume_identifier()?
                    } else {
                        TokenKind::Unknown(c)
                    }
                }
            };
            self.emit(kind);
            self.advance();
        }
        Ok(self.tokens.to_owned())
    }

    fn emit(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            location: Location {
                row: self.row,
                column: self.index.saturating_sub(self.bol),
            },
        })
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }

    fn next(&self) -> Option<char> {
        self.source.get(self.index + 1).copied()
    }

    fn advance(&mut self) {
        self.index += 1;
        if let Some(c) = self.current() {
            if c == '\n' {
                self.row += 1;
                self.bol = self.index + 1;
            }
        }
    }

    fn consume_identifier(&mut self) -> Result<TokenKind> {
        let start = self.index;

        while let Some(c) = self.next() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let identifier: String = self.source[start..=self.index].iter().collect();

        Ok(match identifier.as_str() {
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
        })
    }

    fn consume_number(&mut self) -> Result<TokenKind> {
        let start = self.index;

        while let Some(c) = self.next() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        let value = self.source[start..=self.index]
            .iter()
            .collect::<String>()
            .parse::<i64>();

        match value {
            Ok(v) => Ok(TokenKind::Number(v)),
            Err(_) => Err(Error::NotANumber),
        }
    }
}
