#[warn(clippy::all, clippy::pedantic)]
#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use lemon_lisp::{
        evaluator::Evaluator,
        internal::InternalFunction,
        lexer::TokenStream,
        model::{Environment, Numeric, RuntimeError, Value},
        parser::Parser,
    };
    use rug::Integer;

    fn eval_input(
        input: &str,
        evaluator: &Evaluator,
        environment: &Rc<Environment>,
    ) -> Result<Value, RuntimeError> {
        let token_stream = TokenStream::new(input);
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse()?;

        parse_result
            .into_iter()
            .map(|value| evaluator.eval_value(&value, environment))
            .last()
            .unwrap_or(Ok(Value::Void))
    }

    #[test]
    fn test_define_var() {
        let environment = Environment::new();
        let evaluator = Evaluator;

        let input = "(define a 2)";
        let result = eval_input(input, &evaluator, &environment);

        assert!(result.is_ok());
        assert_eq!(Some(Value::from(Integer::from(2))), environment.get("a"));
    }

    #[test]
    fn test_define_closure() {
        let environment = Environment::new();
        let evaluator = Evaluator;

        let result = eval_input("(define (add-one n) (+ n 1))", &evaluator, &environment);
        assert!(result.is_ok());

        if let Some(Value::Closure(closure)) = environment.get("add-one") {
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
        let environment = Environment::new();
        let evaluator = Evaluator;

        let result = eval_input("(lambda (a b) (+ a b))", &evaluator, &environment);

        assert!(result.is_ok());
        if let Ok(Value::Closure(closure)) = result {
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
        } else {
            panic!("Expected a closure");
        }
    }

    #[test]
    fn test_optimize_tail_call() {
        let environment = Environment::new();
        let evaluator = Evaluator;

        let result = eval_input("(define (loop) (loop))", &evaluator, &environment);

        assert!(result.is_ok());
        if let Some(Value::TailCall(tail_call)) = environment.get("loop") {
            assert_eq!(Some("loop".to_string()), tail_call.closure.name);
            assert!(tail_call.closure.params.is_empty());
            assert!(tail_call.closure.body.is_empty());
            assert!(tail_call.closure.environment.upgrade().is_some());
            assert!(tail_call.updates.is_empty());
            assert_eq!(Value::Bool(false), *tail_call.break_condition);
            assert_eq!(Value::Void, *tail_call.return_expr);
        } else {
            panic!("Expected a tail call");
        }
    }

    #[test]
    fn test_internal_fn() {
        let environment = Environment::new();
        let evaluator = Evaluator;

        let add = |args: &[Value], _: &Rc<Environment>| -> Result<Value, RuntimeError> {
            args.iter()
                .try_fold(Numeric::Integer(0.into()), |acc, arg| {
                    arg.try_as_numeric().map(|n| n + acc)
                })
                .map(Into::into)
        };

        environment.set(
            "+",
            Value::InternalFunction(InternalFunction {
                name: "+".to_string(),
                function: add,
            }),
        );

        let result = eval_input("(+ 2 3)", &evaluator, &environment);

        assert_eq!(Ok(Value::from(Integer::from(5))), result);
    }
}
