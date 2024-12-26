use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Eq, PartialEq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Ampersand,
    Bang,
    Caret,
    Pipe,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equals,
    Less,
    Greater,
    At,
    Colon,
    Comma,
    Dollar,
    Dot,
    Hash,
    Newline,
    QuestionMark,
    Range,
    Semicolon,
    If,
    Elif,
    Else,
    For,
    While,
    Do,
    Loop,
    Fn,
    Return,
    True,
    False,
    Null,
    Identifier(String),
    Number(i64),
    Unknown(char),
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}
