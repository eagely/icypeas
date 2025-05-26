use super::Located;
use super::LocatedExt;
use super::Location;
use super::TokenKind;
use super::TokenValue;
use crate::err;
use crate::error::{Error, ErrorKind, Result};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl LocatedExt<Self> for Token {
    fn at(self, location: Rc<Location>) -> super::Located<Self> {
        Located {
            node: self,
            location,
        }
    }
}

impl Display for Located<Token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}",
            self.node.kind, self.node.value, self.location
        )
    }
}

impl TryFrom<&Located<Token>> for String {
    type Error = Error;

    fn try_from(value: &Located<Token>) -> Result<Self> {
        if let TokenValue::String(string) = &value.node.value {
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
