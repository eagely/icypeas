pub mod builtins;
pub mod environment;

pub use environment::Environment;

use crate::err;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::Lexer;
use crate::model::{Expression, Located, Statement, TokenKind, TokenValue, Value};
use crate::parser::Parser;
use std::cell::RefCell;
use std::convert::TryInto;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    current_file: Option<PathBuf>,
}

impl Interpreter {
    pub const fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            environment,
            current_file: None,
        }
    }

    pub const fn with_file(environment: Rc<RefCell<Environment>>, file: Option<PathBuf>) -> Self {
        Self {
            environment,
            current_file: file,
        }
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
                let value = self.evaluate(expression)?;
                println!("Value({})", self.force(value)?);
                Ok(())
            }
            Statement::Use { path } => {
                let mut relative_path = String::new();
                for (i, part) in path.iter().enumerate() {
                    if let TokenValue::Identifier(ref s) = part.node.value {
                        if i > 0 {
                            relative_path.push('/');
                        }
                        relative_path.push_str(s);
                    } else {
                        return Err(Error::with_help(
                            ErrorKind::InvalidArguments,
                            part.location.clone(),
                            "Import path must be identifiers",
                        ));
                    }
                }
                relative_path.push_str(".icy");

                let base_dir = self
                    .current_file
                    .as_ref()
                    .and_then(|p| p.parent().map(std::path::Path::to_path_buf))
                    .unwrap_or_else(|| PathBuf::from("."));
                let file_path = base_dir.join(&relative_path);

                let source = std::fs::read_to_string(&file_path).map_err(|_| {
                    Error::with_help(
                        ErrorKind::InvalidArguments,
                        path[0].location.clone(),
                        format!("Could not read import file: {}", file_path.display()),
                    )
                })?;

                let mut lexer = Lexer::new();
                let tokens = lexer.lex(&source)?;
                let mut parser = Parser::new();
                let ast = parser.parse(tokens)?;

                let prev_file = self.current_file.take();
                self.current_file = Some(file_path);
                let result = self.interpret(ast);
                self.current_file = prev_file;
                result
            }
            Statement::Variable { name, body } => {
                let name: String = name.node.get_identifier_name().ok_or_else(|| {
                    Error::with_help(
                        ErrorKind::InvalidToken,
                        statement.location.clone(),
                        "Function name must be an identifier",
                    )
                })?;

                let value = self.evaluate(body)?;
                self.environment.borrow_mut().set(name, value);
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
                    match self.force(value)? {
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
                    match self.force(value)? {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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
                    match (self.force(left_value)?, self.force(right_value)?) {
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

                match self.force(function_value)? {
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
                        let thunk = Value::Thunk {
                            expression: *argument,
                            environment: self.environment.clone(),
                        };
                        function_environment.borrow_mut().set(parameter_name, thunk);

                        self.environment = function_environment;
                        let res = self.evaluate(body)?;
                        self.environment = old_environment;

                        Ok(res)
                    }
                    Value::BuiltinFunction { function } => {
                        let value = self.evaluate(*argument)?;
                        function(value, location)
                    }
                    _ => err!(
                        ErrorKind::ExpectedExpression,
                        location,
                        "Tried to invoke a non-function type",
                    ),
                }
            }
            Expression::Identifier { token } => match &token.node.value {
                TokenValue::Identifier(name) => {
                    let value = self.environment.borrow().get(name).ok_or_else(|| {
                        Error::new(ErrorKind::InvalidIdentifier, token.location.clone())
                    })?;
                    self.force(value)
                }
                _ => err!(ErrorKind::UnsupportedExpression, token.location.clone()),
            },
            Expression::If {
                branches,
                otherwise,
            } => {
                for (condition, expression) in branches {
                    let value = self.evaluate(*condition)?;
                    if matches!(self.force(value)?, Value::Boolean(true)) {
                        return self.evaluate(*expression);
                    }
                }
                self.evaluate(*otherwise)
            }
            Expression::Lambda { parameter, body } => Ok(Value::Function {
                parameter,
                body: *body,
                environment: Environment::with_parent(self.environment.clone()),
            }),
            Expression::Literal { token } => (&token).try_into(),
        }
    }

    fn force(&mut self, value: Value) -> Result<Value> {
        match value {
            Value::Thunk {
                expression,
                environment,
            } => {
                let old_environment = self.environment.clone();
                self.environment = environment;
                let value = self.evaluate(expression)?;
                self.environment = old_environment;
                self.force(value)
            }
            other => Ok(other),
        }
    }
}
