use std::{cell::RefCell, rc::Rc};

use crate::model::{Environment, RuntimeError, Value};

pub mod math;

#[derive(Debug, PartialEq, Clone)]
pub struct InternalFunction {
    pub name: String,
    pub function: Function,
}

pub type Function =
    fn(args: &[Value], env: &Rc<RefCell<Environment>>) -> Result<Value, RuntimeError>;
