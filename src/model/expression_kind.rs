use super::{Expression, Token};

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
        function: Box<Expression>,
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
        parameter: Token,
        body: Box<Expression>,
    },
    Literal {
        token: Token,
    },
}
