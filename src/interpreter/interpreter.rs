use super::enums::Value;
use crate::error::Result;
use crate::parser::enums::{Expression, ExpressionKind};

pub struct Interpreter {
    expression: Expression,
}

impl Interpreter {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }

    pub fn interpret(&self) -> Result<Value> {
        self.evaluate(&self.expression)
    }

    pub fn evaluate(&self, expression: &Expression) -> Result<Value> {
        match &expression.kind {
            ExpressionKind::Assignment {
                identifier,
                parameters,
                expression,
            } => {
                todo!();
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
            ExpressionKind::Declaration { name, types } => {
                todo!();
            }
            ExpressionKind::Identifier { token } => {
                todo!();
            }
            ExpressionKind::If {
                branches,
                otherwise,
            } => {
                todo!();
            }
            ExpressionKind::Lambda { parameters, body } => {
                todo!();
            }
            ExpressionKind::Literal { token } => {
                todo!();
            }
            ExpressionKind::Print { expression } => {
                todo!();
            }
        }
    }
}
