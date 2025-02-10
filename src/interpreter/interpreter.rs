use super::enums::Value;
use super::environment::Environment;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::TokenValue;
use crate::parser::enums::{Expression, ExpressionKind};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    expression: Expression,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(expression: Expression, environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            expression,
            environment,
        }
    }

    pub fn interpret(&self) -> Result<Value> {
        self.evaluate(&self.expression)
    }

    pub fn evaluate(&self, expression: &Expression) -> Result<Value> {
        match &expression.kind {
            ExpressionKind::Assignment {
                identifier,
                parameters,
                expression,
            } => {
                todo!();
            }
            ExpressionKind::Unary {
                operator,
                expression,
            } => {
                todo!();
            }
            ExpressionKind::Binary { lhs, operator, rhs } => {
                todo!();
            }
            ExpressionKind::Declaration { name, types } => {
                todo!();
            }
            ExpressionKind::Identifier { token } => match &token.value {
                TokenValue::Identifier(name) => self.environment.borrow().get(name).ok_or(
                    Error::new(ErrorKind::InvalidIdentifier, Rc::clone(&token.location)),
                ),
                _ => Err(Error::new(
                    ErrorKind::UnsupportedExpression,
                    Rc::clone(&token.location),
                )),
            },
            ExpressionKind::If {
                branches,
                otherwise,
            } => {
                let mut v = None;
                let mut else_branch = true;
                for branch in branches {
                    match self.evaluate(&branch.0)? {
                        Value::Boolean(b) => {
                            if b && else_branch {
                                v = Some(self.evaluate(&branch.1));
                                else_branch = false;
                            }
                        }
                        _ => {}
                    }
                }
                if else_branch {
                    if let Some(o) = otherwise {
                        v = Some(self.evaluate(&o));
                    }
                }
                v.ok_or(Error::new(
                    ErrorKind::IncompleteIf,
                    Rc::clone(&expression.location),
                ))?
            }
            ExpressionKind::Lambda { parameters, body } => {
                todo!();
            }
            ExpressionKind::Literal { token } => token.try_into(),
            ExpressionKind::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(value)
            }
        }
    }
}
