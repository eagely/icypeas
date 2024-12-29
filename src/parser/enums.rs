use crate::lexer::enums::Token;

pub enum Expression {
    Assign {
        identifier: Token,
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
    Function {
        name: Token,
        types: Vec<Token>,
        parameters: Vec<Token>,
    },
    If {
        condition: Box<Expression>,
        expression: Box<Expression>,
        otherwise: Option<Box<Expression>>,
    },
    Literal {
        token: Token,
    },
    Print {
        expression: Box<Expression>,
    },
}
