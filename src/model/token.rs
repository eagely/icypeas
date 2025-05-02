use super::Location;
use super::TokenKind;
use super::TokenValue;
use crate::err;
use crate::error::{Error, ErrorKind, Result};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

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
