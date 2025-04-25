use crate::lexer::enums::{Location, Token};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    Assignment {
        name: Token,
        parameter: Token,
        body: Box<Expression>,
    },
    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        function: Token,
        argument: Box<Expression>,
    },
    Declaration {
        name: Token,
        types: Vec<Token>,
    },
    Identifier {
        token: Token,
    },
    If {
        branches: Vec<(Box<Expression>, Box<Expression>)>,
        otherwise: Option<Box<Expression>>,
    },
    Lambda {
        parameters: Vec<Token>,
        body: Box<Expression>,
    },
    Literal {
        token: Token,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: Rc<Location>,
}

impl Expression {
    pub fn new(kind: ExpressionKind, location: Rc<Location>) -> Self {
        Self { kind, location }
    }
}
