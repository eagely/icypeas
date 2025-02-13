use super::{builtins::add_builtins, enums::Value};
use crate::error::Result;
use crate::lexer::enums::Location;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Environment {
    identifiers: HashMap<String, Value>,
    builtins: HashMap<String, fn(Vec<Value>, Rc<Location>) -> Result<Value>>,
    parent: Option<Rc<RefCell<Self>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
            builtins: add_builtins(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            identifiers: HashMap::new(),
            builtins: add_builtins(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.identifiers.get(key).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().get(key))
        })
    }

    pub fn get_builtin(&self, key: &str) -> Option<fn(Vec<Value>, Rc<Location>) -> Result<Value>> {
        self.builtins.get(key).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().get_builtin(key))
        })
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.identifiers.insert(key, value);
    }
}
