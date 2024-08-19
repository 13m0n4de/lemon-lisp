use crate::model::{Closure, Keyword, TailCall, Value};

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

pub fn optimize_closure(closure: Closure) -> Value {
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
            extract_self_call_params(then_expr, function_name)
                .map(|then_params| TailCallInfo::Conditional {
                    updates: then_params,
                    break_condition: Value::List(vec![
                        Value::Symbol("not".into()),
                        condition.clone(),
                    ]),
                    return_expr: else_expr.clone(),
                })
                .or_else(|| {
                    extract_self_call_params(else_expr, function_name).map(|else_params| {
                        TailCallInfo::Conditional {
                            updates: else_params,
                            break_condition: condition.clone(),
                            return_expr: then_expr.clone(),
                        }
                    })
                })
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
