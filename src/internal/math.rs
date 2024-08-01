use std::{cell::RefCell, rc::Rc};

use rug::Float;

use crate::model::{Environment, Numeric, RuntimeError, Value};

pub fn add(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (+ num1 num2 num3) => 0 + num1 + num2 + num3
    let result = args
        .iter()
        .try_fold(Numeric::Integer(0.into()), |acc, arg| {
            arg.try_as_numeric().map(|n| acc + n)
        })?;

    Ok(result.into())
}

pub fn sub(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (- num) => 0 - num
    // (- num1 num2 num3) => num1 - num2 - num3
    if args.len() == 1 {
        let result = args[0]
            .try_as_numeric()
            .map(|n| Numeric::Integer(0.into()) - n)?;
        Ok(result.into())
    } else {
        let result = args
            .iter()
            .skip(1)
            .try_fold(args[0].try_as_numeric()?, |acc, arg| {
                arg.try_as_numeric().map(|n| acc - n)
            })?;
        Ok(result.into())
    }
}

pub fn mul(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (* num1 num2 num3) => 1 * num1 * num2 * num3
    let result = args
        .iter()
        .try_fold(Numeric::Integer(1.into()), |acc, arg| {
            arg.try_as_numeric().map(|n| acc * n)
        })?;
    Ok(result.into())
}

pub fn div(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (/ num) => 1 / num
    // (/ num1 num2 num3) => num1 / num2 / num3
    if args.len() == 1 {
        let result = args[0]
            .try_as_numeric()
            .map(|n| Numeric::Float(Float::with_val(53, 1.0)) / n)?;
        Ok(result.into())
    } else {
        let result = args
            .iter()
            .skip(1)
            .try_fold(args[0].try_as_numeric()?, |acc, arg| {
                arg.try_as_numeric().map(|n| acc / n)
            })?;
        Ok(result.into())
    }
}
