use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{RuntimeError, Value};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    vars: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Self::default();
        Rc::new(RefCell::new(env))
    }

    pub fn extend(parent: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            vars: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|e| e.borrow().get(name)),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn update(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().update(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(name.into()))
        }
    }
}
