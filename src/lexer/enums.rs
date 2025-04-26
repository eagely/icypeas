use crate::error::{Error, ErrorKind, Result};
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.row + 1, self.column + 1)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub location: Rc<Location>,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}, {:?}", self.kind, self.value, self.location)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}, {:?}", self.kind, self.value, self.location)
    }
}

impl TryFrom<&Token> for String {
    type Error = Error;

    fn try_from(value: &Token) -> Result<Self> {
        if let TokenValue::String(string) = &value.value {
            Ok(string.clone())
        } else {
            Err(Error::with_help(
                ErrorKind::InvalidToken,
                Rc::clone(&value.location),
                "This token was expected to be a string",
            ))
        }
    }
}

impl Token {
    pub fn get_identifier_name(&self) -> Option<String> {
        if let TokenValue::Identifier(name) = &self.value {
            Some(name.to_string())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    Percent,
    Ampersand,
    Caret,
    Pipe,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
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
    Float,
    Integer,
    String,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Identifier(String),
    Boolean(bool),
    Float(f64),
    Integer(i128),
    String(String),
    Unknown(char),
    None,
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
            | TokenKind::Integer
            | TokenKind::String => true,
            _ => false,
        }
    }
}
