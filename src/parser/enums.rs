use crate::lexer::enums::Token;

pub enum Expression {
    Assignment {
        identifier: Token,
        parameters: Option<Vec<Token>>,
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
    If {
        condition: Box<Expression>,
        expression: Box<Expression>,
        otherwise: Option<Box<Expression>>,
    },
    Lambda {
        parameters: Option<Vec<Token>>,
        body: Box<Expression>,
    },
    Literal {
        token: Token,
    },
    Print {
        expression: Box<Expression>,
    },
}
