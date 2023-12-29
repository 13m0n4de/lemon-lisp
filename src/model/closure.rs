use std::{cell::RefCell, rc::Rc};

use super::{Environment, RuntimeError, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Value>,
    pub environment: Rc<RefCell<Environment>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TailRecursiveClosure {
    pub closure: Closure,
    pub updates: Vec<Value>,
    pub break_condition: Box<Value>,
    pub return_expr: Box<Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InternalFunction {
    pub name: String,
    pub function: Function,
}

pub type Function =
    fn(args: &[Value], env: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError>;
