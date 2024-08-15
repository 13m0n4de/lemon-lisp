use std::{cell::RefCell, rc::Rc};

use super::{Environment, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Value>,
    pub environment: Rc<RefCell<Environment>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TailCall {
    pub closure: Closure,
    pub updates: Vec<Value>,
    pub break_condition: Box<Value>,
    pub return_expr: Box<Value>,
}
