use super::{Location, statement_kind::StatementKind};
use std::rc::Rc;

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub location: Rc<Location>,
}

impl Statement {
    pub const fn new(kind: StatementKind, location: Rc<Location>) -> Self {
        Self { kind, location }
    }
}
