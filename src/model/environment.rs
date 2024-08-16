use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use super::{RuntimeError, Value};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    parent: Option<Weak<Environment>>,
    vars: RefCell<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    pub fn extend(parent: &Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            vars: RefCell::new(HashMap::new()),
            parent: Some(Rc::downgrade(parent)),
        })
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.vars.borrow().get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|weak| weak.upgrade())
                .and_then(|parent| parent.get(name))
        })
    }

    pub fn set(&self, name: &str, value: Value) {
        self.vars.borrow_mut().insert(name.to_string(), value);
    }

    pub fn update(&self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.vars.borrow_mut().contains_key(name) {
            self.vars.borrow_mut().insert(name.to_string(), value);
            Ok(())
        } else if let Some(weak) = self.parent.as_ref()
            && let Some(parent) = weak.upgrade()
        {
            parent.update(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(name.into()))
        }
    }
}
