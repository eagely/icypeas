use super::enums::Value;
use super::environment::Environment;
use crate::err;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::{TokenKind, TokenValue};
use crate::parser::enums::{Expression, ExpressionKind};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, expression: Expression) -> Result<Value> {
        self.evaluate(expression)
    }

    pub fn evaluate(&mut self, expression: Expression) -> Result<Value> {
        match expression.kind {
            ExpressionKind::Assignment {
                name,
                parameter,
                body,
            } => {
                let name: String = name.get_identifier_name().ok_or(Error::with_help(
                    ErrorKind::InvalidToken,
                    expression.location.clone(),
                    "Function name must be an identifier",
                ))?;

                self.environment
                    .borrow_mut()
                    .set(name, Value::Function { parameter, body });
                Ok(Value::None)
            }
            ExpressionKind::Unary {
                operator,
                expression,
            } => match operator.kind {
                TokenKind::Bang => {
                    let value = self.evaluate(*expression)?;
                    match value {
                        Value::Boolean(b) => Ok(Value::Boolean(!b)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid type for logical NOT",
                        ),
                    }
                }
                TokenKind::Minus => {
                    let value = self.evaluate(*expression)?;
                    match value {
                        Value::Integer(i) => Ok(Value::Integer(-i)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid type for negation",
                        ),
                    }
                }
                _ => err!(
                    ErrorKind::UnsupportedExpression,
                    operator.location.clone(),
                    format!("Unsupported operator: {:?}", operator.kind),
                ),
            },
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => match operator.kind {
                TokenKind::Plus => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l + r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for addition",
                        ),
                    }
                }
                TokenKind::Minus => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l - r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for subtraction",
                        ),
                    }
                }
                TokenKind::Star => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l * r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for multiplication",
                        ),
                    }
                }
                TokenKind::StarStar => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => {
                            let Ok(exp) = u32::try_from(r) else {
                                return if (0..=1).contains(&l) {
                                    Ok(Value::Integer(l))
                                } else {
                                    Err(if r > 0 {
                                        Error::with_help(
                                            ErrorKind::Overflow,
                                            operator.location.clone(),
                                            "Exponent too large",
                                        )
                                    } else {
                                        Error::with_help(
                                            ErrorKind::InvalidArguments,
                                            operator.location.clone(),
                                            "Exponent must be a non-negative integer",
                                        )
                                    })
                                };
                            };
                            Ok(Value::Integer(l.checked_pow(exp).ok_or(Error::new(
                                ErrorKind::Overflow,
                                operator.location.clone(),
                            ))?))
                        }
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for exponentiation",
                        ),
                    }
                }
                TokenKind::Slash => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => {
                            if r == 0 {
                                err!(ErrorKind::DivisionByZero, operator.location.clone())
                            } else {
                                Ok(Value::Integer(l / r))
                            }
                        }
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for division",
                        ),
                    }
                }
                TokenKind::Percent => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => {
                            if r == 0 {
                                err!(ErrorKind::DivisionByZero, operator.location.clone())
                            } else {
                                Ok(Value::Integer(l % r))
                            }
                        }
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for modulo",
                        ),
                    }
                }
                TokenKind::Ampersand => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l & r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l & r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for logical AND",
                        ),
                    }
                }
                TokenKind::Pipe => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l | r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l | r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for logical OR",
                        ),
                    }
                }
                TokenKind::Caret => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l ^ r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l ^ r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for logical XOR",
                        ),
                    }
                }
                TokenKind::BangEqual => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l != r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for inequality",
                        ),
                    }
                }
                TokenKind::EqualEqual => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l == r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for equality",
                        ),
                    }
                }
                TokenKind::Greater => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l > r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l > r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for greater than",
                        ),
                    }
                }
                TokenKind::GreaterEqual => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l >= r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l >= r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for greater than or equal to",
                        ),
                    }
                }
                TokenKind::Less => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l < r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l < r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for less than",
                        ),
                    }
                }
                TokenKind::LessEqual => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l <= r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l <= r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location.clone(),
                            "Invalid types for less than or equal to",
                        ),
                    }
                }
                _ => err!(
                    ErrorKind::UnsupportedExpression,
                    operator.location.clone(),
                    format!("Unsupported operator: {:?}", operator.kind),
                ),
            },
            ExpressionKind::Call { function, argument } => {
                let location = function.location.clone();
                let function_value = self.evaluate(*function)?;

                match function_value {
                    Value::Function { parameter, body } => {
                        let parameter_name = parameter.get_identifier_name().ok_or(Error::new(
                            ErrorKind::InvalidToken,
                            parameter.location.clone(),
                        ))?;
                        let evaluated_argument = self.evaluate(*argument)?;
                        self.environment
                            .borrow_mut()
                            .set(parameter_name, evaluated_argument);
                        Ok(self.evaluate(*body)?)
                    }
                    _ => err!(
                        ErrorKind::ExpectedExpression,
                        location,
                        format!("Tried to invoke a non-function type {:?}", function_value),
                    ),
                }
            }
            ExpressionKind::Declaration { name, types } => {
                todo!()
            }
            ExpressionKind::Identifier { token } => match &token.value {
                TokenValue::Identifier(name) => self.environment.borrow().get(name).ok_or(
                    Error::new(ErrorKind::InvalidIdentifier, token.location.clone()),
                ),
                _ => err!(ErrorKind::UnsupportedExpression, token.location.clone()),
            },
            ExpressionKind::If {
                branches,
                otherwise,
            } => {
                let mut v = None;
                let mut else_branch = true;
                for branch in branches {
                    match self.evaluate(*branch.0)? {
                        Value::Boolean(b) => {
                            if b && else_branch {
                                v = Some(self.evaluate(*branch.1));
                                else_branch = false;
                            }
                        }
                        _ => {}
                    }
                }
                if else_branch {
                    if let Some(o) = otherwise {
                        v = Some(self.evaluate(*o));
                    }
                }
                v.ok_or(Error::new(
                    ErrorKind::IncompleteIf,
                    expression.location.clone(),
                ))?
            }
            ExpressionKind::Lambda { parameter, body } => Ok(Value::Function { parameter, body }),
            ExpressionKind::Literal { token } => (&token).try_into(),
        }
    }
}
