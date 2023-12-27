#[cfg(test)]
mod tests {
    use lemon_lisp::{
        evaluator::Evaluator,
        lexer::TokenStream,
        model::{Environment, Lambda, Value},
        parser::Parser,
    };

    #[test]
    fn test_define_var() {
        let token_stream = TokenStream::new("(define a 2)");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        let environment = Environment::new();
        let evaluator = Evaluator;
        for value in parse_result.unwrap() {
            let evaluate_result = evaluator.evaluate_with_envrionment(&value, environment.clone());
            assert_eq!(Ok(Value::Void), evaluate_result);
        }

        assert_eq!(
            Some(Value::Integer(2.into())),
            environment.borrow().get("a")
        );
    }

    #[test]
    fn test_define_lambda() {
        let token_stream = TokenStream::new("(define (add-one n) (+ n 1))");
        let mut parser = Parser::new(token_stream);
        let parse_result = parser.parse();

        assert!(parse_result.is_ok());

        let environment = Environment::new();
        let evaluator = Evaluator;
        for value in parse_result.unwrap() {
            let evaluate_result = evaluator.evaluate_with_envrionment(&value, environment.clone());
            assert_eq!(Ok(Value::Void), evaluate_result);
        }

        let lambda = Value::Lambda(Lambda {
            params: vec!["n".to_string()],
            body: vec![Value::List(vec![
                Value::Symbol("+".into()),
                Value::Symbol("n".into()),
                Value::Integer(1.into()),
            ])],
            environment: Environment::new(),
        });

        assert_eq!(Some(lambda), environment.borrow().get("add-one"));
    }
}
