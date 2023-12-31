use std::{cell::RefCell, rc::Rc};

use crate::model::{Environment, Numeric, RuntimeError, Value};

pub fn add(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (+ num1 num2 num3) => 0 + num1 + num2 + num3
    let result = args
        .iter()
        .try_fold(Numeric::Integer(0.into()), |acc, arg| {
            arg.try_as_numeric().map(|n| n + acc)
        })?;

    Ok(result.into())
}
