use crate::model::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Environment {
    identifiers: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Self>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            identifiers: HashMap::new(),
            parent: None,
        }))
    }

    pub fn with_parent(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            identifiers: HashMap::new(),
            parent: Some(parent),
        }))
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.identifiers.get(key).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().get(key))
        })
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.identifiers.insert(key, value);
    }
}
