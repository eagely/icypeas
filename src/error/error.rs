use crate::model::Location;
use std::{fmt::Display, rc::Rc};

#[macro_export]
macro_rules! err {
    ($kind:expr, $location:expr $(,)?) => {
        Err($crate::error::Error::new($kind, $location))
    };
    ($kind:expr, $location:expr, $help:expr $(,)?) => {
        Err($crate::error::Error::with_help($kind, $location, $help))
    };
}

#[derive(Debug)]
pub enum ErrorKind {
    DivisionByZero,
    ExpectedExpression,
    IncompleteIf,
    InvalidArguments,
    InvalidIdentifier,
    InvalidToken,
    MismatchedTypes,
    MissingClosingParenthesis,
    MissingParameter,
    NotANumber,
    Overflow,
    UndeclaredFunction,
    UnexpectedEndOfFile,
    UnexpectedToken,
    UnimplementedFunction,
    UnsupportedExpression,
    UnterminatedString,
    UnterminatedUse,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub location: Option<Rc<Location>>,
    pub help: Option<String>,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::DivisionByZero => "Division by zero",
            Self::ExpectedExpression => "Expected expression",
            Self::IncompleteIf => "Incomplete if",
            Self::InvalidArguments => "Invalid arguments",
            Self::InvalidIdentifier => "Invalid identifier",
            Self::InvalidToken => "Invalid token",
            Self::MismatchedTypes => "Mismatched types",
            Self::MissingClosingParenthesis => "Missing closing parenthesis",
            Self::MissingParameter => "Missing parameter",
            Self::NotANumber => "Not a number",
            Self::Overflow => "Overflow",
            Self::UndeclaredFunction => "Undeclared function",
            Self::UnexpectedEndOfFile => "Unexpected end of file",
            Self::UnexpectedToken => "Unexpected token",
            Self::UnimplementedFunction => "Unimplemented function",
            Self::UnsupportedExpression => "Unsupported Expression",
            Self::UnterminatedString => "Unterminated string",
            Self::UnterminatedUse => "Unterminated use",
        };
        write!(f, "{message}")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location_str = &self.location.as_ref().map_or_else(
            || "at unknown location (probably end of file)".to_string(),
            |location| format!("at {location}"),
        );

        if let Some(help) = &self.help {
            write!(f, "{} {}\nHelp: {}", self.kind, location_str, help)
        } else {
            write!(f, "{} {}", self.kind, location_str)
        }
    }
}

impl Error {
    pub const fn new(kind: ErrorKind, location: Rc<Location>) -> Self {
        Self {
            kind,
            location: Some(location),
            help: None,
        }
    }

    pub fn with_help(kind: ErrorKind, location: Rc<Location>, help: impl Into<String>) -> Self {
        Self {
            kind,
            location: Some(location),
            help: Some(help.into()),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            location: None,
            help: None,
        }
    }
}
