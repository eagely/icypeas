use super::{Expression, Token};

#[derive(Debug)]
pub enum StatementKind {
    Declaration {
        name: Token,
        types: Vec<Token>,
    },
    Definition {
        name: Token,
        parameter: Token,
        body: Expression,
    },
    Expression {
        expression: Expression,
    },
}
