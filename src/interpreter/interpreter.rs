use super::enums::Value;
use super::environment::Environment;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::TokenValue;
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
                    Rc::clone(&expression.location),
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
            } => {
                todo!();
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                todo!();
            }
            ExpressionKind::Call { function, argument } => {
                let function_name = function.get_identifier_name().ok_or(Error::with_help(
                    ErrorKind::InvalidToken,
                    function.location.clone(),
                    format!("Expected Identifier, got {:?}", function.kind),
                ))?;

                let function_value =
                    self.environment
                        .borrow()
                        .get(&function_name)
                        .ok_or(Error::new(
                            ErrorKind::UndeclaredFunction,
                            function.location.clone(),
                        ))?;

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
                    _ => Err(Error::with_help(
                        ErrorKind::ExpectedExpression,
                        function.location.clone(),
                        format!("Tried to invoke a non-function type {:?}", function_value),
                    )),
                }
            }
            ExpressionKind::Declaration { name, types } => {
                todo!()
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
                    Rc::clone(&expression.location),
                ))?
            }
            ExpressionKind::Lambda { parameters, body } => {
                todo!();
            }
            ExpressionKind::Literal { token } => (&token).try_into(),
        }
    }
}
