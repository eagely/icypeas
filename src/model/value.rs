use super::Located;
use crate::err;
use crate::error::{ErrorKind, Result};
use crate::interpreter::environment::Environment;
use crate::model::Expression;
use crate::model::{Token, TokenValue};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    Integer(i128),
    None,
    String(String),
    Function {
        parameter: Located<Token>,
        body: Located<Expression>,
        environment: Rc<RefCell<Environment>>,
    },
}

impl TryFrom<&Located<Token>> for Value {
    type Error = crate::error::Error;
    fn try_from(value: &Located<Token>) -> Result<Self> {
        match &value.node.value {
            TokenValue::Boolean(boolean) => Ok(Self::Boolean(*boolean)),
            TokenValue::Float(float) => Ok(Self::Float(*float)),
            TokenValue::Integer(integer) => Ok(Self::Integer(*integer)),
            TokenValue::None => Ok(Self::None),
            TokenValue::String(string) => Ok(Self::String(string.clone())),
            _ => err!(
                ErrorKind::InvalidToken,
                value.location.clone(),
                "Token could not be converted into a value.",
            ),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::Integer(integer) => write!(f, "{integer}"),
            Self::None => write!(f, "None"),
            Self::String(string) => write!(f, "{string}"),
            Self::Function {
                parameter, body, ..
            } => write!(f, "λ{parameter:#?}.{body:#?}"),
        }
    }
}
