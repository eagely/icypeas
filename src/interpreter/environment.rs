use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::enums::Value;

#[derive(Debug)]
pub struct Environment {
    identifiers: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Self>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            identifiers: HashMap::new(),
            parent: Some(parent),
        }
    }
}
