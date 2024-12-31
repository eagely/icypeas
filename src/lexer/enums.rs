use std::fmt::{write, Debug, Display, Formatter, Result};

#[derive(Clone, PartialEq, Eq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
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
    Semicolon,
    Underscore,
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
    Identifier,
    Number,
    String,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenValue {
    Identifier(String),
    Number(i128),
    String(String),
    Unknown(char),
    None,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}:{:?}", self.row + 1, self.column + 1)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}, {:?}, {:?}", self.kind, self.value, self.location)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}, {:?}, {:?}", self.kind, self.value, self.location)
    }
}

impl TokenKind {
    pub fn is_primary(&self) -> bool {
        match self {
            TokenKind::If
            | TokenKind::Elif
            | TokenKind::Else
            | TokenKind::For
            | TokenKind::While
            | TokenKind::Do
            | TokenKind::Loop
            | TokenKind::Fn
            | TokenKind::Return
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Null
            | TokenKind::Identifier
            | TokenKind::Number
            | TokenKind::String => true,
            _ => false,
        }
    }
}
