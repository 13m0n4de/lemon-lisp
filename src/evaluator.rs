use std::{cell::RefCell, rc::Rc};

use crate::model::{Environment, Keyword, Lambda, RuntimeError, Value};

#[derive(Default)]
pub struct Evaluator;

type ValueResult = Result<Value, RuntimeError>;

impl Evaluator {
    pub fn eval_value(&self, value: &Value, env: Rc<RefCell<Environment>>) -> ValueResult {
        match value {
            Value::Void => Ok(Value::Void),
            Value::Lambda { .. } => Ok(Value::Void),
            Value::Symbol(symbol) => self.eval_symbol(symbol, env),
            Value::List(list) => self.eval_list(list, env),
            Value::Quoted(box_value) => Ok(*box_value.clone()),
            _ => Ok(value.clone()),
        }
    }

    fn eval_symbol(&self, symbol: &String, env: Rc<RefCell<Environment>>) -> ValueResult {
        let value = env
            .borrow()
            .get(symbol.as_str())
            .ok_or(RuntimeError::UndefinedVariable(symbol.into()))?;
        Ok(value)
    }

    fn eval_list(&self, list: &[Value], env: Rc<RefCell<Environment>>) -> ValueResult {
        let (first, rest) = list.split_first().ok_or(RuntimeError::EmptyList)?;
        match first {
            Value::Lambda { .. } => {
                let params: Vec<Value> = rest
                    .iter()
                    .map(|value| self.eval_value(value, env.clone()))
                    .try_collect()?;
                self.eval_lambda(first, &params, env)
            }
            Value::Symbol(_) | Value::List(_) => {
                let value = self.eval_value(first, env.clone())?;
                match value {
                    Value::Lambda { .. } => self.eval_lambda(&value, rest, env),
                    Value::TailRecursion {
                        lambda,
                        updates,
                        break_condition,
                        return_expr,
                    } => self.eval_tail_recursion(
                        lambda,
                        updates,
                        *break_condition,
                        *return_expr,
                        rest,
                        env,
                    ),
                    _ => Err(RuntimeError::NonCallableValue(value.clone())),
                }
            }
            Value::Keyword(keyword) => match keyword {
                Keyword::Define => self.eval_keyword_define(rest, env.clone()),
                Keyword::Lambda => self.eval_keyword_lambda(rest, env.clone()),
            },
            _ => Err(RuntimeError::NonCallableValue(first.clone())),
        }
    }

    fn eval_lambda(
        &self,
        lambda: &Value,
        args: &[Value],
        env: Rc<RefCell<Environment>>,
    ) -> ValueResult {
        if let Value::Lambda(Lambda {
            name: _name,
            params,
            body,
            environment: lambda_env,
        }) = lambda
        {
            let new_env = Environment::extend(lambda_env.clone());
            for (i, param) in params.iter().enumerate() {
                let arg = self.eval_value(&args[i], env.clone())?;
                new_env.borrow_mut().set(param, arg);
            }

            let (last_expr, preceding_expr) = body.split_last().unwrap();
            for expr in preceding_expr {
                self.eval_value(expr, new_env.clone())?;
            }
            self.eval_value(last_expr, new_env)
        } else {
            Err(RuntimeError::NonCallableValue(lambda.clone()))
        }
    }

    fn eval_keyword_define(&self, list: &[Value], env: Rc<RefCell<Environment>>) -> ValueResult {
        match list {
            [Value::Symbol(name), value] => {
                env.borrow_mut()
                    .set(name, self.eval_value(value, env.clone())?);
                Ok(Value::Void)
            }
            [Value::List(lambda_info), body @ ..] => {
                let (first, rest) = lambda_info.split_first().ok_or(RuntimeError::EmptyList)?;
                let name = first.try_as_symbol()?;
                let params: Vec<String> = rest
                    .iter()
                    .map(|x| x.try_as_symbol().map(String::from))
                    .try_collect()?;

                let lambda = optimize_lambda(Lambda {
                    name: Some(name.to_string()),
                    params,
                    body: body.to_vec(),
                    environment: Rc::new((*env).clone()),
                })?;

                env.borrow_mut().set(name, lambda);
                Ok(Value::Void)
            }
            [value, ..] => Err(RuntimeError::TypeError {
                expected: "symbol or list",
                founded: value.clone(),
            }),
            [] => Err(RuntimeError::EmptyList),
        }
    }

    fn eval_keyword_lambda(&self, list: &[Value], env: Rc<RefCell<Environment>>) -> ValueResult {
        match list {
            [Value::List(first), body @ ..] => {
                let params: Vec<String> = first
                    .iter()
                    .map(|x| x.try_as_symbol().map(String::from))
                    .try_collect()?;

                let lambda = Lambda {
                    name: None,
                    params,
                    body: body.to_vec(),
                    environment: Rc::new((*env).clone()),
                };

                Ok(Value::Lambda(lambda))
            }
            [Value::Symbol(first), body @ ..] => {
                let params = vec![first.to_string()];

                let lambda = Value::Lambda(Lambda {
                    name: None,
                    params,
                    body: body.to_vec(),
                    environment: Rc::new((*env).clone()),
                });

                Ok(lambda)
            }
            [value, ..] => Err(RuntimeError::TypeError {
                expected: "symbol or list",
                founded: value.clone(),
            }),
            [] => Err(RuntimeError::EmptyList),
        }
    }

    fn eval_tail_recursion(
        &self,
        lambda: Lambda,
        updates: Vec<Value>,
        break_condition: Value,
        return_expr: Value,
        args: &[Value],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        let new_env = Environment::extend(lambda.environment);
        for (i, param) in lambda.params.iter().enumerate() {
            let arg = self.eval_value(&args[i], env.clone())?;
            new_env.borrow_mut().set(param, arg);
        }

        for expr in &lambda.body {
            self.eval_value(expr, new_env.clone())?;
        }

        loop {
            let args: Vec<Value> = updates
                .iter()
                .map(|update| self.eval_value(update, new_env.clone()))
                .try_collect()?;
            lambda.params.iter().zip(args).for_each(|(param, arg)| {
                new_env.borrow_mut().set(param, arg);
            });

            for expr in &lambda.body {
                self.eval_value(expr, new_env.clone())?;
            }
            if self
                .eval_value(&break_condition, new_env.clone())?
                .try_as_bool()?
            {
                break self.eval_value(&return_expr, new_env);
            }
        }
    }
}

fn optimize_lambda(lambda: Lambda) -> ValueResult {
    if let Some((last_expr, preceding_expr)) = lambda.body.split_last() &&
        let Value::List(last_list) = last_expr
    {
        match last_list.as_slice() {
            [Value::Symbol(symbol), params @ ..] if Some(symbol) == lambda.name.as_ref() => {
                Ok(Value::TailRecursion {
                    lambda: Lambda {
                        name: lambda.name,
                        params: params
                            .iter()
                            .map(|p| p.try_as_symbol().map(String::from))
                            .try_collect()?,
                        body: preceding_expr.to_vec(),
                        environment: lambda.environment
                    },
                    updates: params.to_vec(),
                    break_condition: Value::Bool(false).into(),
                    return_expr: Value::Void.into(),
                })
            }
            // [Value::Keyword(Keyword::If)] => todo!(),
            _ => Ok(Value::Lambda(lambda))
        }
    } else {
        Ok(Value::Lambda(lambda))
    }
}
