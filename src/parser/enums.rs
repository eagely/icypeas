use std::rc::Rc;

use crate::lexer::enums::{Location, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    Assignment {
        identifier: Token,
        parameters: Vec<Token>,
        expression: Box<Expression>,
    },
    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
    Binary {
        lhs: Box<Expression>,
        operator: Token,
        rhs: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
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
