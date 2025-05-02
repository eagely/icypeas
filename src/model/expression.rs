use super::ExpressionKind;
use crate::model::Location;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: Rc<Location>,
}

impl Expression {
    pub const fn new(kind: ExpressionKind, location: Rc<Location>) -> Self {
        Self { kind, location }
    }
}
