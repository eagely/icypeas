use crate::err;
use crate::error::{Error, ErrorKind, Result};
use crate::model::{Located, LocatedExt, Location, Token, TokenKind, TokenValue};
use std::rc::Rc;

pub struct Lexer {
    source: Vec<char>,
    index: usize,
    row: usize,
    bol: usize,
}

impl Lexer {
    pub const fn new() -> Self {
        Self {
            source: vec![],
            index: 0,
            row: 0,
            bol: 0,
        }
    }

    pub fn lex(&mut self, source: &str) -> Result<Vec<Located<Token>>> {
        self.source = source.chars().collect();
        let mut tokens = vec![];
        while let Some(c) = self.current() {
            if c.is_whitespace() && c != '\n' {
                self.advance();
                continue;
            }
            tokens.push(self.consume_token(c)?.at(self.location()));
            self.advance();
        }
        Ok(tokens)
    }

    fn consume(&mut self, c: char) -> bool {
        if matches!(self.next(1), Some(cc) if cc == c) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }

    fn next(&self, n: usize) -> Option<char> {
        self.source.get(self.index + n).copied()
    }

    const fn advance(&mut self) {
        self.index += 1;
    }

    fn location(&self) -> Rc<Location> {
        Rc::new(Location {
            row: self.row,
            column: self.index.saturating_sub(self.bol),
        })
    }

    fn consume_token(&mut self, c: char) -> Result<Token> {
        Ok(Token::new(
            match c {
                '\n' => {
                    self.row += 1;
                    self.bol = self.index + 1;
                    TokenKind::Newline
                }
                '{' => self.consume_comment(),
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                '(' => TokenKind::LeftParenthesis,
                ')' => TokenKind::RightParenthesis,
                '&' => TokenKind::Ampersand,
                '!' => {
                    if self.consume('=') {
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    }
                }
                '^' => TokenKind::Caret,
                '|' => TokenKind::Pipe,
                '+' => TokenKind::Plus,
                '-' => {
                    if self.consume('-') {
                        while let Some(c) = self.current() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        TokenKind::Newline
                    } else {
                        TokenKind::Minus
                    }
                }
                '*' => {
                    if self.consume('*') {
                        TokenKind::StarStar
                    } else {
                        TokenKind::Star
                    }
                }
                '/' => TokenKind::Slash,
                '%' => TokenKind::Percent,
                '=' => {
                    if self.consume('=') {
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    }
                }
                '<' => {
                    if self.consume('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    if self.consume('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    }
                }
                '@' => TokenKind::At,
                ':' => TokenKind::Colon,
                ',' => TokenKind::Comma,
                '$' => TokenKind::Dollar,
                '.' => TokenKind::Dot,
                '#' => TokenKind::Hash,
                '?' => TokenKind::QuestionMark,
                ';' => TokenKind::Semicolon,
                '_' => TokenKind::Underscore,
                '"' => return self.consume_string(),
                _ => {
                    return if c.is_ascii_digit() {
                        self.consume_number()
                    } else if c.is_alphabetic() || c == '_' {
                        Ok(self.consume_identifier())
                    } else {
                        Ok(Token::new(TokenKind::Unknown, TokenValue::Unknown(c)))
                    };
                }
            },
            TokenValue::None,
        ))
    }

    fn consume_comment(&mut self) -> TokenKind {
        if self.consume('-') {
            let mut nesting = 1;
            loop {
                match (self.current(), self.next(1)) {
                    (Some('{'), Some('-')) => {
                        nesting += 1;
                        self.advance();
                        self.advance();
                    }
                    (Some('-'), Some('}')) => {
                        nesting -= 1;
                        self.advance();
                        self.advance();
                    }
                    _ => self.advance(),
                }
                if nesting == 0 {
                    break;
                }
            }
            TokenKind::Newline
        } else {
            TokenKind::LeftBrace
        }
    }

    fn consume_identifier(&mut self) -> Token {
        let start = self.index;

        while let Some(c) = self.next(1) {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let identifier: String = self.source[start..=self.index].iter().collect();

        match identifier.as_str() {
            "if" => Token::new(TokenKind::If, TokenValue::None),
            "then" => Token::new(TokenKind::Then, TokenValue::None),
            "elif" => Token::new(TokenKind::Elif, TokenValue::None),
            "else" => Token::new(TokenKind::Else, TokenValue::None),
            "true" => Token::new(TokenKind::True, TokenValue::Boolean(true)),
            "false" => Token::new(TokenKind::False, TokenValue::Boolean(false)),
            "null" => Token::new(TokenKind::Null, TokenValue::None),
            "use" => Token::new(TokenKind::Use, TokenValue::None),
            _ => Token::new(TokenKind::Identifier, TokenValue::Identifier(identifier)),
        }
    }

    fn consume_number(&mut self) -> Result<Token> {
        let start = self.index;

        while let Some(c) = self.next(1) {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        if self.consume('.') {
            match self.next(2) {
                Some(c) if c.is_ascii_digit() => {
                    while let Some(c) = self.next(1) {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        self.advance();
                    }

                    let number = self.source[start..=self.index].iter().collect::<String>();

                    return Ok(Token::new(
                        TokenKind::Float,
                        TokenValue::Float(
                            number
                                .parse()
                                .map_err(|_| Error::new(ErrorKind::NotANumber, self.location()))?,
                        ),
                    ));
                }
                _ => (),
            }
        }

        let number = self.source[start..=self.index].iter().collect::<String>();

        Ok(Token::new(
            TokenKind::Integer,
            TokenValue::Integer(
                number
                    .parse()
                    .map_err(|_| Error::new(ErrorKind::NotANumber, self.location()))?,
            ),
        ))
    }

    fn consume_string(&mut self) -> Result<Token> {
        let start = self.index + 1;
        while let Some(c) = self.next(1) {
            self.advance();
            if c == '"' {
                return Ok(Token::new(
                    TokenKind::String,
                    TokenValue::String(self.source[start..self.index].iter().collect()),
                ));
            }
        }
        err!(
            ErrorKind::UnterminatedString,
            self.location(),
            "Expected a \" after this string.",
        )
    }
}
