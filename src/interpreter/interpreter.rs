use super::environment::Environment;
use crate::err;
use crate::error::{Error, ErrorKind, Result};
use crate::model::{Expression, Located, Statement, TokenKind, TokenValue, Value};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub const fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, statements: Vec<Located<Statement>>) -> Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: Located<Statement>) -> Result<()> {
        match statement.node {
            Statement::Declaration { name, types } => {
                todo!()
            }
            Statement::Definition {
                name,
                parameter,
                body,
            } => {
                let name: String = name.node.get_identifier_name().ok_or_else(|| {
                    Error::with_help(
                        ErrorKind::InvalidToken,
                        statement.location.clone(),
                        "Function name must be an identifier",
                    )
                })?;

                let environment = Environment::with_parent(self.environment.clone());
                self.environment.borrow_mut().set(
                    name,
                    Value::Function {
                        parameter,
                        body,
                        environment,
                    },
                );
                Ok(())
            }
            Statement::Expression { expression } => {
                println!("{}", self.evaluate(expression)?);
                Ok(())
            }
        }
    }

    fn evaluate(&mut self, expression: Located<Expression>) -> Result<Value> {
        match expression.node {
            Expression::Unary {
                operator,
                expression,
            } => match operator.node.kind {
                TokenKind::Bang => {
                    let value = self.evaluate(*expression)?;
                    match value {
                        Value::Boolean(b) => Ok(Value::Boolean(!b)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                            operator.location,
                            "Invalid type for negation",
                        ),
                    }
                }
                _ => err!(
                    ErrorKind::UnsupportedExpression,
                    operator.location.clone(),
                    format!("Unsupported operator: {:?}", operator.node.kind),
                ),
            },
            Expression::Binary {
                left,
                operator,
                right,
            } => match operator.node.kind {
                TokenKind::Plus => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l + r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                            operator.location,
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
                            operator.location,
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
                                            operator.location,
                                            "Exponent too large",
                                        )
                                    } else {
                                        Error::with_help(
                                            ErrorKind::InvalidArguments,
                                            operator.location,
                                            "Exponent must be a non-negative integer",
                                        )
                                    })
                                };
                            };
                            Ok(Value::Integer(l.checked_pow(exp).ok_or_else(|| {
                                Error::new(ErrorKind::Overflow, operator.location)
                            })?))
                        }
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                                err!(ErrorKind::DivisionByZero, operator.location)
                            } else {
                                Ok(Value::Integer(l / r))
                            }
                        }
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                                err!(ErrorKind::DivisionByZero, operator.location)
                            } else {
                                Ok(Value::Integer(l % r))
                            }
                        }
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                            operator.location,
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
                            operator.location,
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
                            operator.location,
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
                            operator.location,
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
                            operator.location,
                            "Invalid types for equality",
                        ),
                    }
                }
                TokenKind::Greater => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l > r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l && !r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                            operator.location,
                            "Invalid types for greater than or equal to",
                        ),
                    }
                }
                TokenKind::Less => {
                    let left_value = self.evaluate(*left)?;
                    let right_value = self.evaluate(*right)?;
                    match (left_value, right_value) {
                        (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l < r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(!l & r)),
                        _ => err!(
                            ErrorKind::InvalidArguments,
                            operator.location,
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
                            operator.location,
                            "Invalid types for less than or equal to",
                        ),
                    }
                }
                _ => err!(
                    ErrorKind::UnsupportedExpression,
                    operator.location.clone(),
                    format!("Unsupported operator: {:?}", operator.node.kind),
                ),
            },
            Expression::Call { function, argument } => {
                let location = function.location.clone();
                let function_value = self.evaluate(*function)?;

                match function_value {
                    Value::Function {
                        parameter,
                        body,
                        environment,
                    } => {
                        let old_environment = self.environment.clone();
                        let function_environment = Environment::with_parent(environment);

                        let parameter_name =
                            parameter.node.get_identifier_name().ok_or_else(|| {
                                Error::new(ErrorKind::InvalidToken, parameter.location)
                            })?;
                        let evaluated_argument = self.evaluate(*argument)?;
                        function_environment
                            .borrow_mut()
                            .set(parameter_name, evaluated_argument);

                        self.environment = function_environment;
                        let res = self.evaluate(body)?;
                        self.environment = old_environment;

                        Ok(res)
                    }
                    _ => err!(
                        ErrorKind::ExpectedExpression,
                        location,
                        format!("Tried to invoke a non-function type {:?}", function_value),
                    ),
                }
            }
            Expression::Identifier { token } => match &token.node.value {
                TokenValue::Identifier(name) => {
                    self.environment.borrow().get(name).ok_or_else(|| {
                        Error::new(ErrorKind::InvalidIdentifier, token.location.clone())
                    })
                }
                _ => err!(ErrorKind::UnsupportedExpression, token.location.clone()),
            },
            Expression::If {
                branches,
                otherwise,
            } => {
                let true_branch = branches
                    .into_iter()
                    .map(|(cond, expr)| match self.evaluate(*cond)? {
                        Value::Boolean(true) => Ok(Some(*expr)),
                        _ => Ok(None),
                    })
                    .find_map(Result::transpose)
                    .transpose()?;

                if let Some(b) = true_branch {
                    self.evaluate(b)
                } else {
                    self.evaluate(*otherwise)
                }
            }
            Expression::Lambda { parameter, body } => Ok(Value::Function {
                parameter,
                body: *body,
                environment: Environment::with_parent(self.environment.clone()),
            }),
            Expression::Literal { token } => (&token).try_into(),
        }
    }
}
