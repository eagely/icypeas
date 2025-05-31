pub mod builtins;
pub mod environment;

pub use environment::Environment;

use crate::err;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::Lexer;
use crate::model::{Expression, Located, Statement, Token, TokenKind, TokenValue, Value};
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
            Statement::Declaration { .. } => {
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
            } => self.evaluate_unary(operator, *expression),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(*left, operator, *right),
            Expression::Call { function, argument } => self.evaluate_call(*function, *argument),
            Expression::Identifier { token } => self.evaluate_identifier(&token),
            Expression::If {
                branches,
                otherwise,
            } => self.evaluate_if(branches, *otherwise),
            Expression::Lambda { parameter, body } => self.evaluate_lambda(parameter, *body),
            Expression::Literal { token } => (&token).try_into(),
        }
    }

    fn evaluate_unary(
        &mut self,
        operator: Located<Token>,
        expression: Located<Expression>,
    ) -> Result<Value> {
        match operator.node.kind {
            TokenKind::Bang => {
                let value = self.evaluate(expression)?;
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
                let value = self.evaluate(expression)?;
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
        }
    }

    fn evaluate_binary(
        &mut self,
        left: Located<Expression>,
        operator: Located<Token>,
        right: Located<Expression>,
    ) -> Result<Value> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
        let left_forced = self.force(left_value)?;
        let right_forced = self.force(right_value)?;

        match (operator.node.kind, left_forced, right_forced) {
            (TokenKind::Plus, Value::Integer(l), Value::Integer(r)) => l
                .checked_add(r)
                .map(Value::Integer)
                .ok_or_else(|| Error::new(ErrorKind::Overflow, operator.location.clone())),
            (TokenKind::Plus, Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),

            (TokenKind::Minus, Value::Integer(l), Value::Integer(r)) => l
                .checked_sub(r)
                .map(Value::Integer)
                .ok_or_else(|| Error::new(ErrorKind::Overflow, operator.location.clone())),

            (TokenKind::Star, Value::Integer(l), Value::Integer(r)) => l
                .checked_mul(r)
                .map(Value::Integer)
                .ok_or_else(|| Error::new(ErrorKind::Overflow, operator.location.clone())),

            (TokenKind::StarStar, Value::Integer(l), Value::Integer(r)) => {
                let exp = match u32::try_from(r) {
                    Ok(exp) => exp,
                    Err(_) if (0..=1).contains(&l) => return Ok(Value::Integer(l)),
                    Err(_) => {
                        return if r > 0 {
                            err!(ErrorKind::Overflow, operator.location, "Exponent too large")
                        } else {
                            err!(
                                ErrorKind::InvalidArguments,
                                operator.location,
                                "Exponent must be non-negative"
                            )
                        };
                    }
                };
                l.checked_pow(exp)
                    .map(Value::Integer)
                    .ok_or_else(|| Error::new(ErrorKind::Overflow, operator.location.clone()))
            }

            (TokenKind::Slash, Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    err!(ErrorKind::DivisionByZero, operator.location)
                } else {
                    Ok(Value::Integer(l / r))
                }
            }

            (TokenKind::Percent, Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    err!(ErrorKind::DivisionByZero, operator.location)
                } else {
                    Ok(Value::Integer(l % r))
                }
            }

            (TokenKind::Ampersand, Value::Integer(l), Value::Integer(r)) => {
                Ok(Value::Integer(l & r))
            }
            (TokenKind::Ampersand, Value::Boolean(l), Value::Boolean(r)) => {
                Ok(Value::Boolean(l & r))
            }

            (TokenKind::Pipe, Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l | r)),
            (TokenKind::Pipe, Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l | r)),

            (TokenKind::Caret, Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l ^ r)),
            (TokenKind::Caret, Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l ^ r)),

            (TokenKind::BangEqual, Value::Integer(l), Value::Integer(r)) => {
                Ok(Value::Boolean(l != r))
            }
            (TokenKind::BangEqual, Value::Boolean(l), Value::Boolean(r)) => {
                Ok(Value::Boolean(l != r))
            }

            (TokenKind::EqualEqual, Value::Integer(l), Value::Integer(r)) => {
                Ok(Value::Boolean(l == r))
            }
            (TokenKind::EqualEqual, Value::Boolean(l), Value::Boolean(r)) => {
                Ok(Value::Boolean(l == r))
            }

            (TokenKind::Greater, Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l > r)),
            (TokenKind::Greater, Value::Boolean(l), Value::Boolean(r)) => {
                Ok(Value::Boolean(l && !r))
            }

            (TokenKind::GreaterEqual, Value::Integer(l), Value::Integer(r)) => {
                Ok(Value::Boolean(l >= r))
            }
            (TokenKind::GreaterEqual, Value::Boolean(l), Value::Boolean(r)) => {
                Ok(Value::Boolean(l >= r))
            }

            (TokenKind::Less, Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l < r)),
            (TokenKind::Less, Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(!l & r)),

            (TokenKind::LessEqual, Value::Integer(l), Value::Integer(r)) => {
                Ok(Value::Boolean(l <= r))
            }
            (TokenKind::LessEqual, Value::Boolean(l), Value::Boolean(r)) => {
                Ok(Value::Boolean(l <= r))
            }

            (op, left, right) => err!(
                ErrorKind::InvalidArguments,
                operator.location,
                format!("{:?} and {:?} have invalid types for {:?}", left, right, op),
            ),
        }
    }

    fn evaluate_call(
        &mut self,
        function: Located<Expression>,
        argument: Located<Expression>,
    ) -> Result<Value> {
        let location = function.location.clone();
        let function_value = self.evaluate(function)?;

        match self.force(function_value)? {
            Value::Function {
                parameter,
                body,
                environment,
            } => {
                let old_environment = self.environment.clone();
                let function_environment = Environment::with_parent(environment);

                let parameter_name = parameter
                    .node
                    .get_identifier_name()
                    .ok_or_else(|| Error::new(ErrorKind::InvalidToken, parameter.location))?;
                let thunk = Value::Thunk {
                    expression: argument,
                    environment: self.environment.clone(),
                };
                function_environment.borrow_mut().set(parameter_name, thunk);

                self.environment = function_environment;
                let res = self.evaluate(body)?;
                self.environment = old_environment;

                Ok(res)
            }
            Value::BuiltinFunction { function } => {
                let value = self.evaluate(argument)?;
                function(value, location)
            }
            _ => err!(
                ErrorKind::ExpectedExpression,
                location,
                "Tried to invoke a non-function type",
            ),
        }
    }

    fn evaluate_identifier(&mut self, token: &Located<Token>) -> Result<Value> {
        match &token.node.value {
            TokenValue::Identifier(name) => {
                let value = self.environment.borrow().get(name).ok_or_else(|| {
                    Error::new(ErrorKind::InvalidIdentifier, token.location.clone())
                })?;
                self.force(value)
            }
            _ => err!(ErrorKind::UnsupportedExpression, token.location.clone()),
        }
    }

    fn evaluate_if(
        &mut self,
        branches: Vec<(Located<Expression>, Located<Expression>)>,
        otherwise: Located<Expression>,
    ) -> Result<Value> {
        for (condition, expression) in branches {
            let value = self.evaluate(condition)?;
            if matches!(self.force(value)?, Value::Boolean(true)) {
                return self.evaluate(expression);
            }
        }
        self.evaluate(otherwise)
    }

    fn evaluate_lambda(
        &self,
        parameter: Located<Token>,
        body: Located<Expression>,
    ) -> Result<Value> {
        if TokenKind::Identifier == parameter.node.kind {
            Ok(Value::Function {
                parameter,
                body,
                environment: Environment::with_parent(self.environment.clone()),
            })
        } else {
            err!(
                ErrorKind::InvalidToken,
                parameter.location,
                "Expected an identifier"
            )
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
