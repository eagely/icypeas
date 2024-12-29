use super::enums::{Location, Token, TokenKind};
use crate::error::{Error, Result};

pub struct Lexer {
    source: Vec<char>,
    index: usize,
    row: usize,
    bol: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            index: 0,
            row: 0,
            bol: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(c) = self.current() {
            let kind = match c {
                '\n' => {
                    self.row += 1;
                    self.bol = self.index + 1;
                    TokenKind::Newline
                }
                c if c.is_whitespace() => {
                    self.advance();
                    continue;
                }
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '&' => TokenKind::Ampersand,
                '!' => TokenKind::Bang,
                '^' => TokenKind::Caret,
                '|' => TokenKind::Pipe,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '%' => TokenKind::Percent,
                '=' => TokenKind::Equals,
                '<' => TokenKind::Less,
                '>' => TokenKind::Greater,
                '@' => TokenKind::At,
                ':' => TokenKind::Colon,
                ',' => TokenKind::Comma,
                '$' => TokenKind::Dollar,
                '.' => TokenKind::Dot,
                '#' => TokenKind::Hash,
                '?' => TokenKind::QuestionMark,
                ';' => TokenKind::Semicolon,
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
            tokens.push(Token {
                kind,
                location: Location {
                    row: self.row,
                    column: self.index.saturating_sub(self.bol),
                },
            });
            self.advance();
        }
        Ok(tokens)
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }

    fn next(&self) -> Option<char> {
        self.source.get(self.index + 1).copied()
    }

    fn advance(&mut self) {
        self.index += 1;
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
        let mut float = false;

        while let Some(c) = self.next() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !float {
                float = true;
                self.advance();
                break;
            } else {
                break;
            }
        }
        
        if float {
            self.next()
                .ok_or(Error::UnexpectedEndOfFile)?
                .is_ascii_digit()
                .then_some(())
                .ok_or(Error::NotANumber)?;
        }

        while let Some(c) = self.next() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let number = self.source[start..=self.index].iter().collect::<String>();

        Ok(if float {
            TokenKind::Float(number.parse().map_err(|_| Error::NotANumber)?)
        } else {
            TokenKind::Integer(number.parse().map_err(|_| Error::NotANumber)?)
        })
    }
}
