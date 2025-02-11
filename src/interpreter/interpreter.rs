use super::enums::{FunctionPattern, Value};
use super::environment::Environment;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::TokenValue;
use crate::parser::enums::{Expression, ExpressionKind};
use std::cell::RefCell;
use std::convert::TryInto;
use std::iter::once;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, expression: &Expression) -> Result<Value> {
        self.evaluate(expression)
    }

    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value> {
        match &expression.kind {
            ExpressionKind::Assignment {
                identifier,
                parameters,
                expression,
            } => {
                let name: String = identifier.get_identifier_name().ok_or(Error::with_help(
                    ErrorKind::InvalidToken,
                    Rc::clone(&expression.location),
                    "Function name must be an identifier",
                ))?;

                let function = { self.environment.borrow().get(&name) };
                if let Some(Value::Function {
                    types,
                    patterns,
                    environment,
                }) = function
                {
                    let pattern = FunctionPattern {
                        parameters: parameters.to_owned(),
                        body: Rc::new(*expression.to_owned()),
                    };
                    let function = Value::Function {
                        types,
                        patterns: patterns.into_iter().chain(once(pattern)).collect(),
                        environment,
                    };
                    self.environment.borrow_mut().set(name, function.clone());
                    Ok(function)
                } else {
                    Err(Error::new(
                        ErrorKind::UndeclaredFunction,
                        Rc::clone(&expression.location),
                    ))
                }
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
            ExpressionKind::Call {
                function,
                arguments,
            } => {
                let function_name = if let ExpressionKind::Identifier { token } = &function.kind {
                    if let TokenValue::Identifier(name) = &token.value {
                        name.clone()
                    } else {
                        return Err(Error::with_help(
                            ErrorKind::InvalidIdentifier,
                            Rc::clone(&token.location),
                            "Expected function name to be an identifier",
                        ));
                    }
                } else {
                    return Err(Error::with_help(
                        ErrorKind::InvalidIdentifier,
                        Rc::clone(&expression.location),
                        "Expected function name to be an identifier",
                    ));
                };

                if function_name == "print" {
                    let mut evaluated_arguments = Vec::new();
                    for arg in arguments {
                        evaluated_arguments.push(self.evaluate(arg)?);
                    }

                    for arg in &evaluated_arguments {
                        println!("{:?}", arg);
                    }

                    return Ok(Value::None);
                }
                
                let function =
                    self.environment
                        .borrow()
                        .get(&function_name)
                        .ok_or(Error::with_help(
                            ErrorKind::UndeclaredFunction,
                            Rc::clone(&expression.location),
                            "Tried to call a function that was never declared",
                        ))?;

                if let Value::Function {
                    types,
                    patterns,
                    environment,
                } = function
                {
                    if patterns.is_empty() {
                        return Err(Error::with_help(
                            ErrorKind::UnimplementedFunction,
                            Rc::clone(&expression.location),
                            "This function is declared but not implemented",
                        ));
                    }
                    let mut evaluated_arguments = Vec::new();
                    for arg in arguments {
                        evaluated_arguments.push(self.evaluate(arg)?);
                    }

                    let pattern = patterns
                        .iter()
                        .find(|pattern| pattern.parameters.len() == evaluated_arguments.len());

                    if let Some(pattern) = pattern {
                        let value;
                        {
                            let mut new_env = Environment::with_parent(Rc::clone(&environment));
                            for (param, arg) in pattern.parameters.iter().zip(evaluated_arguments) {
                                let param_name =
                                    param.get_identifier_name().ok_or(Error::with_help(
                                        ErrorKind::InvalidToken,
                                        Rc::clone(&expression.location),
                                        "Parameter name must be an identifier",
                                    ))?;
                                new_env.set(param_name, arg);
                            }

                            let old_env = Rc::clone(&self.environment);
                            self.environment = Rc::new(RefCell::new(new_env));
                            value = self.evaluate(&pattern.body);
                            self.environment = old_env;
                        }
                        value
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidArguments,
                            Rc::clone(&expression.location),
                        ))
                    }
                } else {
                    Err(Error::with_help(
                        ErrorKind::InvalidIdentifier,
                        Rc::clone(&expression.location),
                        "This identifier is not a function",
                    ))
                }
            }
            ExpressionKind::Declaration { name, types } => {
                let name = name.get_identifier_name().ok_or(Error::with_help(
                    ErrorKind::InvalidToken,
                    Rc::clone(&expression.location),
                    "Function names must be identifiers",
                ))?;

                let types: Result<Vec<String>> = types
                    .iter()
                    .map(|t| {
                        t.get_identifier_name().ok_or(Error::with_help(
                            ErrorKind::InvalidToken,
                            Rc::clone(&expression.location),
                            "Function types must be identifiers",
                        ))
                    })
                    .collect();
                let types = types?;

                let function = Value::Function {
                    types,
                    patterns: Vec::new(),
                    environment: Rc::new(RefCell::new(Environment::with_parent(Rc::clone(
                        &self.environment,
                    )))),
                };
                self.environment.borrow_mut().set(name, function.clone());
                Ok(function.clone())
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
        }
    }
}
