use super::enums::{Location, Token, TokenKind, TokenValue};
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
            if c.is_whitespace() && c != '\n' {
                self.advance();
                continue;
            }
            let (kind, value) = self.consume_char(c)?;
            tokens.push(Token {
                kind,
                value,
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

    fn consume_char(&mut self, c: char) -> Result<(TokenKind, TokenValue)> {
        Ok((
            match c {
                '\n' => {
                    self.row += 1;
                    self.bol = self.index + 1;
                    TokenKind::Newline
                }

                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                '(' => TokenKind::LeftParenthesis,
                ')' => TokenKind::RightParenthesis,
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
                '_' => TokenKind::Underscore,
                '"' => return Ok(self.consume_string()?),
                _ => {
                    if c.is_digit(10) {
                        return Ok(self.consume_number()?);
                    } else if c.is_alphabetic() || c == '_' {
                        return Ok(self.consume_identifier()?);
                    } else {
                        return Ok((TokenKind::Unknown, TokenValue::Unknown(c)));
                    }
                }
            },
            TokenValue::None,
        ))
    }

    fn consume_identifier(&mut self) -> Result<(TokenKind, TokenValue)> {
        let start = self.index;

        while let Some(c) = self.next() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let identifier: String = self.source[start..=self.index].iter().collect();

        Ok((
            match identifier.as_str() {
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
                _ => return Ok((TokenKind::Identifier, TokenValue::Identifier(identifier))),
            },
            TokenValue::None,
        ))
    }

    fn consume_number(&mut self) -> Result<(TokenKind, TokenValue)> {
        let start = self.index;

        while let Some(c) = self.next() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        let number = self.source[start..=self.index].iter().collect::<String>();

        Ok((
            TokenKind::Number,
            TokenValue::Number(number.parse().map_err(|_| Error::NotANumber)?),
        ))
    }

    fn consume_string(&mut self) -> Result<(TokenKind, TokenValue)> {
        let start = self.index + 1;
        while let Some(c) = self.next() {
            self.advance();
            if c == '"' {
                return Ok((
                    TokenKind::String,
                    TokenValue::String(self.source[start..self.index].iter().collect()),
                ));
            }
        }
        Err(Error::UnterminatedString)
    }
}
