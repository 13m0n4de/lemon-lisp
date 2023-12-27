use std::{cell::RefCell, rc::Rc};

use crate::model::{Environment, Keyword, Lambda, RuntimeError, Value};

#[derive(Default)]
pub struct Evaluator;

type ValueResult = Result<Value, RuntimeError>;

impl Evaluator {
    #[inline]
    pub fn evaluate(&self, value: &Value) -> ValueResult {
        let environment = Environment::new();
        self.evaluate_with_envrionment(value, environment)
    }

    pub fn evaluate_with_envrionment(
        &self,
        value: &Value,
        env: Rc<RefCell<Environment>>,
    ) -> ValueResult {
        match value {
            Value::Void => Ok(Value::Void),
            Value::Lambda { .. } => Ok(Value::Void),
            Value::Symbol(symbol) => self.evaluate_symbol(symbol, env),
            Value::List(list) => self.evaluate_list(list, env),
            Value::Quoted(box_value) => Ok(*box_value.clone()),
            _ => Ok(value.clone()),
        }
    }

    fn evaluate_symbol(&self, symbol: &String, env: Rc<RefCell<Environment>>) -> ValueResult {
        let value = env
            .borrow()
            .get(symbol.as_str())
            .ok_or(RuntimeError::UndefinedVariable(symbol.into()))?;
        Ok(value)
    }

    fn evaluate_list(&self, list: &[Value], env: Rc<RefCell<Environment>>) -> ValueResult {
        let (first, rest) = list.split_first().ok_or(RuntimeError::EmptyList)?;
        match first {
            Value::Lambda { .. } => {
                let params: Vec<Value> = rest
                    .iter()
                    .map(|value| self.evaluate_with_envrionment(value, env.clone()))
                    .try_collect()?;
                self.evaluate_lambda(first, &params, env)
            }
            Value::Symbol(_) | Value::List(_) => {
                let value = self.evaluate_with_envrionment(first, env.clone())?;
                match value {
                    Value::Lambda { .. } => self.evaluate_lambda(&value, rest, env),
                    _ => Err(RuntimeError::NonCallableValue(value.clone())),
                }
            }
            Value::Keyword(keyword) => match keyword {
                Keyword::Define => self.evaluate_define(rest, env.clone()),
            },
            _ => Err(RuntimeError::NonCallableValue(first.clone())),
        }
    }

    fn evaluate_lambda(
        &self,
        lambda: &Value,
        args: &[Value],
        env: Rc<RefCell<Environment>>,
    ) -> ValueResult {
        if let Value::Lambda(Lambda {
            params,
            body,
            environment: lambda_env,
        }) = lambda
        {
            let new_env = Environment::extend(lambda_env.clone());
            for (i, param) in params.iter().enumerate() {
                let arg = self.evaluate_with_envrionment(&args[i], env.clone())?;
                new_env.borrow_mut().set(param, arg);
            }

            let (last_expr, preceding_expr) = body.split_last().unwrap();
            for expr in preceding_expr {
                self.evaluate_with_envrionment(expr, new_env.clone())?;
            }
            self.evaluate_with_envrionment(last_expr, new_env)
        } else {
            Err(RuntimeError::NonCallableValue(lambda.clone()))
        }
    }

    fn evaluate_define(&self, list: &[Value], env: Rc<RefCell<Environment>>) -> ValueResult {
        match list {
            [Value::Symbol(name), value] => {
                env.borrow_mut()
                    .set(name, self.evaluate_with_envrionment(value, env.clone())?);
                Ok(Value::Void)
            }
            [Value::List(lambda_info), body @ ..] => {
                let (first, rest) = lambda_info.split_first().ok_or(RuntimeError::EmptyList)?;
                let name = first.try_as_symbol()?;
                let params: Vec<String> = rest
                    .iter()
                    .map(|x| x.try_as_symbol().map(String::from))
                    .try_collect()?;

                env.borrow_mut().set(
                    name,
                    Value::Lambda(Lambda {
                        params,
                        body: body.to_vec(),
                        environment: env.clone(),
                    }),
                );
                Ok(Value::Void)
            }
            [value, ..] => Err(RuntimeError::TypeError {
                expected: "symbol",
                founded: value.clone(),
            }),
            [] => Err(RuntimeError::EmptyList),
        }
    }
}
