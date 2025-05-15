use std::rc::Rc;

use super::Location;

#[derive(Clone, Debug)]
pub struct Located<T> {
    pub node: T,
    pub location: Rc<Location>,
}

pub trait LocatedExt<T> {
    fn at(self, location: Rc<Location>) -> Located<T>;
}
