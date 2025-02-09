use std::fmt::Display;

use crate::lexer::enums::Location;

#[derive(Debug)]
pub enum ErrorKind {
    ExpectedExpression,
    InvalidIdentifier,
    InvalidToken,
    MissingClosingParenthesis,
    NotANumber,
    UnexpectedEndOfFile,
    UnsupportedExpression,
    UnterminatedString,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub location: Option<Location>,
    pub help: Option<String>,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::ExpectedExpression => "Expected expression",
            Self::InvalidIdentifier => "Invalid identifier",
            Self::InvalidToken => "Invalid token",
            Self::MissingClosingParenthesis => "Missing closing parenthesis",
            Self::NotANumber => "Not a number",
            Self::UnexpectedEndOfFile => "Unexpected end of file",
            Self::UnsupportedExpression => "Unsupported Expression",
            Self::UnterminatedString => "Unterminated string",
        };
        write!(f, "{}", message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location_str = match &self.location {
            Some(location) => format!("at {}", location),
            None => "at unknown location (probably end of file)".to_string(),
        };

        if let Some(help) = &self.help {
            write!(f, "{} {}\nHelp: {}", self.kind, location_str, help)
        } else {
            write!(f, "{} {}", self.kind, location_str)
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, location: Location) -> Self {
        Self {
            kind,
            location: Some(location),
            help: None,
        }
    }

    pub fn with_help(kind: ErrorKind, location: Location, help: impl Into<String>) -> Self {
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
