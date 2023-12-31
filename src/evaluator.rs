use std::{cell::RefCell, rc::Rc};

use crate::model::{Closure, Environment, Keyword, RuntimeError, TailRecursiveClosure, Value};

#[derive(Default)]
pub struct Evaluator;

type ValueResult = Result<Value, RuntimeError>;

impl Evaluator {
    pub fn eval_value(&self, value: &Value, env: Rc<RefCell<Environment>>) -> ValueResult {
        match value {
            Value::Void => Ok(Value::Void),
            Value::Closure { .. } => Ok(Value::Void),
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
            Value::Closure(closure) => {
                let params: Vec<Value> = rest
                    .iter()
                    .map(|value| self.eval_value(value, env.clone()))
                    .try_collect()?;
                self.eval_closure(closure, &params, env)
            }
            Value::Symbol(_) | Value::List(_) => {
                let value = self.eval_value(first, env.clone())?;
                match value {
                    Value::Closure(closure) => self.eval_closure(&closure, rest, env),
                    Value::TailRecursiveClosure(tail_recursive_closure) => {
                        self.eval_tail_recursive_closure(&tail_recursive_closure, rest, env)
                    }
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

    fn eval_closure(
        &self,
        closure: &Closure,
        args: &[Value],
        env: Rc<RefCell<Environment>>,
    ) -> ValueResult {
        let new_env = Environment::extend(closure.environment.clone());
        for (i, param) in closure.params.iter().enumerate() {
            let arg = self.eval_value(&args[i], env.clone())?;
            new_env.borrow_mut().set(param, arg);
        }

        let (last_expr, preceding_expr) = closure.body.split_last().unwrap();
        for expr in preceding_expr {
            self.eval_value(expr, new_env.clone())?;
        }
        self.eval_value(last_expr, new_env)
    }

    fn eval_tail_recursive_closure(
        &self,
        tail_recursive_closure: &TailRecursiveClosure,
        args: &[Value],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        let TailRecursiveClosure {
            closure,
            updates,
            break_condition,
            return_expr,
        } = tail_recursive_closure;

        let new_env = Environment::extend(closure.environment.clone());
        for (i, param) in closure.params.iter().enumerate() {
            let arg = self.eval_value(&args[i], env.clone())?;
            new_env.borrow_mut().set(param, arg);
        }

        for expr in &closure.body {
            self.eval_value(expr, new_env.clone())?;
        }

        loop {
            let args: Vec<Value> = updates
                .iter()
                .map(|update| self.eval_value(update, new_env.clone()))
                .try_collect()?;
            closure.params.iter().zip(args).for_each(|(param, arg)| {
                new_env.borrow_mut().set(param, arg);
            });

            for expr in &closure.body {
                self.eval_value(expr, new_env.clone())?;
            }
            if self
                .eval_value(break_condition, new_env.clone())?
                .try_as_bool()?
            {
                break self.eval_value(return_expr, new_env);
            }
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

                let closure = Closure {
                    name: Some(name.to_string()),
                    params,
                    body: body.to_vec(),
                    environment: Rc::new((*env).clone()),
                };
                let closure = tail_recursive_optimization(closure)?;

                env.borrow_mut().set(name, closure);
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

                let closure = Closure {
                    name: None,
                    params,
                    body: body.to_vec(),
                    environment: Rc::new((*env).clone()),
                };

                Ok(Value::Closure(closure))
            }
            [Value::Symbol(first), body @ ..] => {
                let params = vec![first.to_string()];

                let closure = Closure {
                    name: None,
                    params,
                    body: body.to_vec(),
                    environment: Rc::new((*env).clone()),
                };

                Ok(Value::Closure(closure))
            }
            [value, ..] => Err(RuntimeError::TypeError {
                expected: "symbol or list",
                founded: value.clone(),
            }),
            [] => Err(RuntimeError::EmptyList),
        }
    }
}

fn tail_recursive_optimization(closure: Closure) -> ValueResult {
    if let Some((last_expr, preceding_expr)) = closure.body.split_last() &&
        let Value::List(last_list) = last_expr
    {
        match last_list.as_slice() {
            [Value::Symbol(symbol), params @ ..] if Some(symbol) == closure.name.as_ref() => {
                let tail_recursive_closure = TailRecursiveClosure {
                    closure: Closure {
                        name: closure.name,
                        params: params
                            .iter()
                            .map(|p| p.try_as_symbol().map(String::from))
                            .try_collect()?,
                        body: preceding_expr.to_vec(),
                        environment: closure.environment
                    },
                    updates: params.to_vec(),
                    break_condition: Value::Bool(false).into(),
                    return_expr: Value::Void.into(),
                };
                Ok(Value::TailRecursiveClosure(tail_recursive_closure))
            }
            // [Value::Keyword(Keyword::If)] => todo!(),
            _ => Ok(Value::Closure(closure))
        }
    } else {
        Ok(Value::Closure(closure))
    }
}
