#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use lemon_lisp::{
        evaluator::Evaluator,
        internal::InternalFunction,
        lexer::TokenStream,
        model::{Closure, Environment, Numeric, RuntimeError, TailCall, Value},
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

        let closure = Closure {
            name: Some("add-one".to_string()),
            params: vec!["n".to_string()],
            body: vec![Value::List(vec![
                Value::Symbol("+".into()),
                Value::Symbol("n".into()),
                Value::from(Integer::from(1)),
            ])],
            environment: Environment::new(),
        };

        assert_eq!(
            Some(Value::Closure(closure)),
            environment.borrow().get("add-one")
        );
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

        assert_eq!(
            Ok(Value::Closure(Closure {
                name: None,
                params: vec!["a".to_string(), "b".to_string()],
                body: vec![Value::List(vec![
                    Value::Symbol("+".into()),
                    Value::Symbol("a".into()),
                    Value::Symbol("b".into()),
                ])],
                environment: Environment::new(),
            })),
            evaluate_result
        );
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

        let tail_recursive_closure = TailCall {
            closure: Closure {
                name: Some("loop".into()),
                params: vec![],
                body: vec![],
                environment: Environment::new(),
            },
            updates: vec![],
            break_condition: Value::Bool(false).into(),
            return_expr: Value::Void.into(),
        };
        assert_eq!(
            Some(Value::TailCall(tail_recursive_closure)),
            environment.borrow().get("loop")
        );
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
