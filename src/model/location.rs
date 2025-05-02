use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.row + 1, self.column + 1)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}
