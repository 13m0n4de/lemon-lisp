use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{Environment, Value};

#[derive(Debug, Clone)]
pub struct Closure {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Value>,
    pub environment: Weak<RefCell<Environment>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TailCall {
    pub closure: Closure,
    pub updates: Vec<Value>,
    pub break_condition: Box<Value>,
    pub return_expr: Box<Value>,
}

impl Closure {
    pub fn new(
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Value>,
        env: &Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            name,
            params,
            body,
            environment: Rc::downgrade(env),
        }
    }
}

impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params && self.body == other.body
    }
}
