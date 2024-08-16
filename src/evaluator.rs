use std::rc::Rc;

use crate::{
    internal::InternalFunction,
    model::{Closure, Environment, Keyword, RuntimeError, TailCall, Value},
};

#[derive(Default)]
pub struct Evaluator;

type EvalResult = Result<Value, RuntimeError>;

impl Evaluator {
    pub fn eval_value(&self, value: &Value, env: &Rc<Environment>) -> EvalResult {
        match value {
            Value::Void | Value::Closure { .. } => Ok(Value::Void),
            Value::Symbol(symbol) => Self::eval_symbol(symbol, env),
            Value::List(list) => self.eval_list(list, env),
            Value::Quoted(box_value) => Ok(*box_value.clone()),
            _ => Ok(value.clone()),
        }
    }

    fn eval_symbol(symbol: &str, env: &Rc<Environment>) -> EvalResult {
        let value = env
            .get(symbol)
            .ok_or(RuntimeError::UndefinedVariable(symbol.into()))?;
        Ok(value)
    }

    fn eval_list(&self, list: &[Value], env: &Rc<Environment>) -> EvalResult {
        let (first, rest) = list.split_first().ok_or(RuntimeError::EmptyList)?;
        match first {
            Value::Closure(closure) => {
                let params: Vec<Value> = rest
                    .iter()
                    .map(|value| self.eval_value(value, env))
                    .try_collect()?;
                self.eval_closure(closure, &params, env)
            }
            Value::Symbol(_) | Value::List(_) => {
                let value = self.eval_value(first, env)?;
                match value {
                    Value::Closure(closure) => self.eval_closure(&closure, rest, env),
                    Value::TailCall(tail_call) => self.eval_tail_call(&tail_call, rest, env),
                    Value::InternalFunction(internal_fn) => {
                        self.eval_internal_fn(&internal_fn, rest, env)
                    }
                    _ => Err(RuntimeError::NonCallableValue(value.clone())),
                }
            }
            Value::Keyword(keyword) => match keyword {
                Keyword::Define => self.eval_keyword_define(rest, env),
                Keyword::Lambda => Self::eval_keyword_lambda(rest, env),
                Keyword::If => self.eval_keyword_if(rest, env),
            },
            _ => Err(RuntimeError::NonCallableValue(first.clone())),
        }
    }

    fn eval_closure(&self, closure: &Closure, args: &[Value], env: &Rc<Environment>) -> EvalResult {
        if closure.params.len() != args.len() {
            return Err(RuntimeError::InvalidArity {
                expected: closure.params.len(),
                founded: args.len(),
            });
        }

        let closure_env = closure
            .environment
            .upgrade()
            .ok_or_else(|| RuntimeError::InvalidClosure)?;
        let new_env = Environment::extend(&closure_env);
        for (i, param) in closure.params.iter().enumerate() {
            let arg = self.eval_value(&args[i], env)?;
            new_env.set(param, arg);
        }

        let (last_expr, preceding_expr) = closure.body.split_last().unwrap();
        for expr in preceding_expr {
            self.eval_value(expr, &new_env)?;
        }
        self.eval_value(last_expr, &new_env)
    }

    fn eval_tail_call(
        &self,
        tail_call: &TailCall,
        args: &[Value],
        env: &Rc<Environment>,
    ) -> Result<Value, RuntimeError> {
        let TailCall {
            closure,
            updates,
            break_condition,
            return_expr,
        } = tail_call;

        if closure.params.len() != args.len() {
            return Err(RuntimeError::InvalidArity {
                expected: closure.params.len(),
                founded: args.len(),
            });
        }

        let closure_env = closure
            .environment
            .upgrade()
            .ok_or_else(|| RuntimeError::InvalidClosure)?;
        let new_env = Environment::extend(&closure_env);
        for (i, param) in closure.params.iter().enumerate() {
            let arg = self.eval_value(&args[i], env)?;
            new_env.set(param, arg);
        }

        for expr in &closure.body {
            self.eval_value(expr, &new_env)?;
        }

        loop {
            let args: Vec<Value> = updates
                .iter()
                .map(|update| self.eval_value(update, &new_env))
                .try_collect()?;
            closure.params.iter().zip(args).for_each(|(param, arg)| {
                new_env.set(param, arg);
            });

            for expr in &closure.body {
                self.eval_value(expr, &new_env)?;
            }
            if self.eval_value(break_condition, &new_env)?.try_as_bool()? {
                break self.eval_value(return_expr, &new_env);
            }
        }
    }

    fn eval_internal_fn(
        &self,
        internal_fn: &InternalFunction,
        args: &[Value],
        env: &Rc<Environment>,
    ) -> EvalResult {
        let args: Vec<Value> = args
            .iter()
            .map(|value| self.eval_value(value, env))
            .try_collect()?;
        (internal_fn.function)(&args, env)
    }

    fn eval_keyword_define(&self, list: &[Value], env: &Rc<Environment>) -> EvalResult {
        match list {
            [Value::Symbol(name), value] => {
                env.set(name, self.eval_value(value, env)?);
                Ok(Value::Void)
            }
            [Value::List(lambda_info), body @ ..] => {
                let (first, rest) = lambda_info.split_first().ok_or(RuntimeError::EmptyList)?;
                let name = first.try_as_symbol()?;
                let params: Vec<String> = rest
                    .iter()
                    .map(|x| x.try_as_symbol().map(String::from))
                    .try_collect()?;

                let closure = Closure::new(Some(name.to_string()), params, body.to_vec(), env);
                let closure = optimize_tail_call(closure);

                env.set(name, closure);
                Ok(Value::Void)
            }
            [value, ..] => Err(RuntimeError::TypeError {
                expected: "symbol or list",
                founded: value.clone(),
            }),
            [] => Err(RuntimeError::EmptyList),
        }
    }

    fn eval_keyword_lambda(list: &[Value], env: &Rc<Environment>) -> EvalResult {
        match list {
            [Value::List(first), body @ ..] => {
                let params: Vec<String> = first
                    .iter()
                    .map(|x| x.try_as_symbol().map(String::from))
                    .try_collect()?;
                let closure = Closure::new(None, params, body.to_vec(), env);
                Ok(Value::Closure(closure))
            }
            [Value::Symbol(first), body @ ..] => {
                let params = vec![first.to_string()];
                let closure = Closure::new(None, params, body.to_vec(), env);
                Ok(Value::Closure(closure))
            }
            [value, ..] => Err(RuntimeError::TypeError {
                expected: "symbol or list",
                founded: value.clone(),
            }),
            [] => Err(RuntimeError::EmptyList),
        }
    }

    fn eval_keyword_if(&self, list: &[Value], env: &Rc<Environment>) -> EvalResult {
        match list {
            [condition, then_expr, else_expr] => {
                let cond_result = self.eval_value(condition, env)?;
                match cond_result {
                    Value::Bool(false) => self.eval_value(else_expr, env),
                    _ => self.eval_value(then_expr, env),
                }
            }
            _ => Err(RuntimeError::InvalidArity {
                expected: 3,
                founded: list.len(),
            }),
        }
    }
}

enum TailCallInfo {
    Direct {
        updates: Vec<Value>,
    },
    Conditional {
        updates: Vec<Value>,
        break_condition: Value,
        return_expr: Value,
    },
}

fn optimize_tail_call(closure: Closure) -> Value {
    let Some(function_name) = &closure.name else {
        return Value::Closure(closure);
    };

    match closure.body.split_last() {
        Some((Value::List(last_list), preceding_expr)) => {
            match detect_tail_call(last_list, function_name) {
                Some(TailCallInfo::Direct { updates }) => Value::from(TailCall {
                    closure: Closure {
                        name: closure.name,
                        params: closure.params,
                        body: preceding_expr.to_vec(),
                        environment: closure.environment,
                    },
                    updates,
                    break_condition: Value::Bool(false).into(),
                    return_expr: Value::Void.into(),
                }),
                Some(TailCallInfo::Conditional {
                    updates,
                    break_condition,
                    return_expr,
                }) => Value::from(TailCall {
                    closure: Closure {
                        name: closure.name,
                        params: closure.params,
                        body: preceding_expr.to_vec(),
                        environment: closure.environment,
                    },
                    updates,
                    break_condition: break_condition.into(),
                    return_expr: return_expr.into(),
                }),
                None => Value::Closure(closure),
            }
        }
        _ => Value::Closure(closure),
    }
}

fn detect_tail_call(expr: &[Value], function_name: &str) -> Option<TailCallInfo> {
    match expr {
        [Value::Symbol(symbol), params @ ..] if symbol == function_name => {
            Some(TailCallInfo::Direct {
                updates: params.to_vec(),
            })
        }
        [Value::Keyword(Keyword::If), condition, then_expr, else_expr] => {
            if let Some(then_params) = extract_self_call_params(then_expr, function_name) {
                Some(TailCallInfo::Conditional {
                    updates: then_params,
                    break_condition: Value::List(vec![
                        Value::Symbol("not".into()),
                        condition.clone(),
                    ]),
                    return_expr: else_expr.clone(),
                })
            } else {
                extract_self_call_params(else_expr, function_name).map(|else_params| {
                    TailCallInfo::Conditional {
                        updates: else_params,
                        break_condition: condition.clone(),
                        return_expr: then_expr.clone(),
                    }
                })
            }
        }
        _ => None,
    }
}

fn extract_self_call_params(expr: &Value, function_name: &str) -> Option<Vec<Value>> {
    if let Value::List(list) = expr {
        if let [Value::Symbol(symbol), params @ ..] = list.as_slice() {
            if symbol == function_name {
                return Some(params.to_vec());
            }
        }
    }
    None
}
