use std::fmt::Display;

use crate::lexer::enums::Location;

#[derive(Debug)]
pub enum ErrorKind {
    ExpectedExpression,
    InvalidIdentifier,
    MissingClosingParenthesis,
    NotANumber,
    UnexpectedEndOfFile,
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
            ErrorKind::ExpectedExpression => "Expected expression",
            ErrorKind::InvalidIdentifier => "Invalid identifier",
            ErrorKind::MissingClosingParenthesis => "Missing closing parenthesis",
            ErrorKind::NotANumber => "Not a number",
            ErrorKind::UnexpectedEndOfFile => "Unexpected end of file",
            ErrorKind::UnterminatedString => "Unterminated string",
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
    pub fn new(kind: ErrorKind, location: Location, help: impl Into<String>) -> Self {
        Self {
            kind,
            location: Some(location),
            help: Some(help.into()),
        }
    }

    pub fn new_without_help(kind: ErrorKind, location: Location) -> Self {
        Self {
            kind,
            location: Some(location),
            help: None,
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
