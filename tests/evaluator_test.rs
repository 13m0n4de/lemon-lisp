#[warn(clippy::all, clippy::pedantic)]
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use lemon_lisp::{
        evaluator::Evaluator,
        internal::InternalFunction,
        lexer::TokenStream,
        model::{Environment, Numeric, RuntimeError, Value},
        parser::Parser,
    };
    use rug::Integer;

    #[test]
    fn test_define_var() {
        let token_stream = TokenStream::new("(define a 2)");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        let environment = Environment::new();
        let evaluator = Evaluator;
        for value in parse_result.unwrap() {
            let evaluate_result = evaluator.eval_value(&value, &environment);
            assert_eq!(Ok(Value::Void), evaluate_result);
        }

        assert_eq!(
            Some(Value::from(Integer::from(2))),
            environment.borrow().get("a")
        );
    }

    #[test]
    fn test_define_closure() {
        let token_stream = TokenStream::new("(define (add-one n) (+ n 1))");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        let environment = Environment::new();
        let evaluator = Evaluator;
        for value in parse_result.unwrap() {
            let evaluate_result = evaluator.eval_value(&value, &environment);
            assert_eq!(Ok(Value::Void), evaluate_result);
        }

        if let Some(Value::Closure(closure)) = environment.borrow().get("add-one") {
            assert_eq!(Some("add-one".to_string()), closure.name);
            assert_eq!(vec!["n".to_string()], closure.params);
            assert_eq!(
                vec![Value::List(vec![
                    Value::Symbol("+".into()),
                    Value::Symbol("n".into()),
                    Value::from(Integer::from(1)),
                ])],
                closure.body
            );
            assert!(closure.environment.upgrade().is_some());
        } else {
            panic!("Expected to find a closure named 'add-one' in the environment");
        };
    }

    #[test]
    fn test_lambda() {
        let token_stream = TokenStream::new("(lambda (a b) (+ a b))");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        let environment = Environment::new();
        let evaluator = Evaluator;
        let value = &parse_result.unwrap()[0];
        let evaluate_result = evaluator.eval_value(value, &environment);

        match evaluate_result {
            Ok(Value::Closure(closure)) => {
                assert_eq!(None, closure.name);
                assert_eq!(vec!["a".to_string(), "b".to_string()], closure.params);
                assert_eq!(
                    vec![Value::List(vec![
                        Value::Symbol("+".into()),
                        Value::Symbol("a".into()),
                        Value::Symbol("b".into()),
                    ])],
                    closure.body
                );
                assert!(closure.environment.upgrade().is_some());
            }
            _ => panic!("Expected a closure"),
        }
    }

    #[test]
    fn test_optimize_tail_call() {
        let token_stream = TokenStream::new("(define (loop) (loop))");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        let environment = Environment::new();
        let evaluator = Evaluator;
        let value = &parse_result.unwrap()[0];
        let evaluate_result = evaluator.eval_value(value, &environment);

        assert_eq!(Ok(Value::Void), evaluate_result);
        match environment.borrow().get("loop") {
            Some(Value::TailCall(tail_call)) => {
                assert_eq!(Some("loop".to_string()), tail_call.closure.name);
                assert!(tail_call.closure.params.is_empty());
                assert!(tail_call.closure.body.is_empty());
                assert!(tail_call.closure.environment.upgrade().is_some());
                assert!(tail_call.updates.is_empty());
                assert_eq!(Value::Bool(false), *tail_call.break_condition);
                assert_eq!(Value::Void, *tail_call.return_expr);
            }
            _ => panic!("Expected a tail call"),
        };
    }

    #[test]
    fn test_internal_fn() {
        let token_stream = TokenStream::new("(+ 2 3)");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        // (+ num1 num2 num3) => 0 + num1 + num2 + num3
        let add = |args: &[Value], _: &Rc<RefCell<Environment>>| -> Result<Value, RuntimeError> {
            let result = args
                .iter()
                .try_fold(Numeric::Integer(0.into()), |acc, arg| {
                    arg.try_as_numeric().map(|n| n + acc)
                })?;

            Ok(result.into())
        };

        let environment = Environment::new();
        environment.borrow_mut().set(
            "+",
            Value::InternalFunction(InternalFunction {
                name: "+".to_string(),
                function: add,
            }),
        );

        let evaluator = Evaluator;
        let value = &parse_result.unwrap()[0];
        let evaluate_result = evaluator.eval_value(value, &environment);

        assert_eq!(Ok(Value::from(Integer::from(5))), evaluate_result);
    }
}
