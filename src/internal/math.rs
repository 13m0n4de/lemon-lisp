use std::{cell::RefCell, rc::Rc};

use rug::Float;

use crate::model::{Environment, Numeric, RuntimeError, Value};

pub fn add(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (+ num1 num2 num3) => 0 + num1 + num2 + num3
    args.iter()
        .try_fold(Numeric::Integer(0.into()), |acc, arg| {
            arg.try_as_numeric().map(|n| acc + n)
        })
        .map(Into::into)
}

pub fn sub(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (- num) => 0 - num
    // (- num1 num2 num3) => num1 - num2 - num3
    match args {
        [] => Err(RuntimeError::InvalidArity {
            expected: 1,
            founded: 0,
        }),
        [single_arg] => single_arg
            .try_as_numeric()
            .map(|n| Numeric::Integer(0.into()) - n)
            .map(Into::into),
        [first_arg, rest @ ..] => rest
            .iter()
            .try_fold(first_arg.try_as_numeric()?, |acc, arg| {
                arg.try_as_numeric().map(|n| acc - n)
            })
            .map(Into::into),
    }
}

pub fn mul(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (* num1 num2 num3) => 1 * num1 * num2 * num3
    args.iter()
        .try_fold(Numeric::Integer(1.into()), |acc, arg| {
            arg.try_as_numeric().map(|n| acc * n)
        })
        .map(Into::into)
}

pub fn div(args: &[Value], _: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    // (/ num) => 1 / num
    // (/ num1 num2 num3) => num1 / num2 / num3
    match args {
        [] => Err(RuntimeError::InvalidArity {
            expected: 1,
            founded: 0,
        }),
        [single_arg] => single_arg
            .try_as_numeric()
            .and_then(|n| {
                if n.is_zero() {
                    Err(RuntimeError::DivideByZero)
                } else {
                    Ok(Numeric::Float(Float::with_val(53, 1.0)) / n)
                }
            })
            .map(Into::into),
        [first_arg, rest @ ..] => {
            let first_numeric = first_arg.try_as_numeric()?;
            let result = rest.iter().try_fold(first_numeric, |acc, arg| {
                arg.try_as_numeric().and_then(|n| {
                    if n.is_zero() {
                        Err(RuntimeError::DivideByZero)
                    } else {
                        Ok(acc / n)
                    }
                })
            });
            Ok(result?.into())
        }
    }
}
