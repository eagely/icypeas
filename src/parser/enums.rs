use crate::lexer::enums::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
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
    Print {
        expression: Box<Expression>,
    },
}
