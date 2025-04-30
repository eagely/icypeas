use crate::err;
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
            err!(
                ErrorKind::InvalidToken,
                value.location.clone(),
                "This token was expected to be a string",
            )
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub const fn is_primary(self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Elif
                | Self::Else
                | Self::For
                | Self::While
                | Self::Do
                | Self::Loop
                | Self::Fn
                | Self::Return
                | Self::True
                | Self::False
                | Self::Null
                | Self::Identifier
                | Self::Integer
                | Self::String
        )
    }

    pub const fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Ampersand
                | Self::Caret
                | Self::Pipe
                | Self::Plus
                | Self::Minus
                | Self::Star
                | Self::StarStar
                | Self::Slash
                | Self::Percent
                | Self::BangEqual
                | Self::Equal
                | Self::EqualEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
                | Self::At
                | Self::Colon
                | Self::Hash
        )
    }
}
