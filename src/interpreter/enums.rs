use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::{Token, TokenValue};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    Integer(i128),
    String(String),
}

impl TryFrom<&Token> for Value {
    type Error = crate::error::Error;
    fn try_from(value: &Token) -> Result<Self> {
        match &value.value {
            TokenValue::Boolean(boolean) => Ok(Self::Boolean(boolean.clone())),
            TokenValue::Float(float) => Ok(Self::Float(float.clone())),
            TokenValue::Integer(integer) => Ok(Value::Integer(integer.clone())),
            TokenValue::String(string) => Ok(Value::String(string.clone())),
            _ => Err(Error::with_help(
                ErrorKind::InvalidToken,
                Rc::clone(&value.location),
                "Token could not be converted into a value.",
            )),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(integer) => write!(f, "{}", integer),
            Value::Float(float) => write!(f, "{}", float),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::String(string) => write!(f, "{}", string),
        }
    }
}
