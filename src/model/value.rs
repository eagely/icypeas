use super::{Located, Location};
use crate::err;
use crate::error::{ErrorKind, Result};
use crate::interpreter::environment::Environment;
use crate::model::Expression;
use crate::model::{Token, TokenValue};
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone)]
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
    BuiltinFunction {
        function: Rc<dyn Fn(Value, Rc<Location>) -> Result<Value>>,
    },
    Thunk {
        expression: Located<Expression>,
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

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "Boolean({b:?})"),
            Self::Float(fl) => write!(f, "Float({fl:?})"),
            Self::Integer(i) => write!(f, "Integer({i:?})"),
            Self::None => write!(f, "None"),
            Self::String(s) => write!(f, "String({s:?})"),
            Self::Function {
                parameter, body, ..
            } => write!(
                f,
                "Function {{ parameter: {parameter:?}, body: {body:?}, ... }}"
            ),
            Self::BuiltinFunction { .. } => write!(f, "BuiltinFunction"),
            Self::Thunk { expression, .. } => {
                write!(f, "Thunk {{ expression: {expression:?}, ... }}")
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::None => write!(f, "None"),
            Self::String(s) => write!(f, "{s}"),
            Self::Function {
                parameter, body, ..
            } => write!(
                f,
                "Function {{ parameter: {parameter}, body: {body:?}, ... }}"
            ),
            Self::BuiltinFunction { .. } => write!(f, "BuiltinFunction"),
            Self::Thunk { expression, .. } => {
                write!(f, "Thunk {{ expression: {expression:?}, ... }}")
            }
        }
    }
}
