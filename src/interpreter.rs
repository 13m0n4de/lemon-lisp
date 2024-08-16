use std::rc::Rc;

use crate::{
    evaluator::Evaluator,
    internal::{math, InternalFunction},
    lexer::TokenStream,
    model::{Environment, RuntimeError, Value},
    parser::Parser,
};

#[derive(Default)]
pub struct Interpreter {
    environment: Rc<Environment>,
    evaluator: Evaluator,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Self::initialize_environment(),
            evaluator: Evaluator,
        }
    }

    fn initialize_environment() -> Rc<Environment> {
        let env = Environment::new();

        env.set(
            "+",
            Value::InternalFunction(InternalFunction {
                name: "+".to_string(),
                function: math::add,
            }),
        );
        env.set(
            "-",
            Value::InternalFunction(InternalFunction {
                name: "-".to_string(),
                function: math::sub,
            }),
        );
        env.set(
            "*",
            Value::InternalFunction(InternalFunction {
                name: "*".to_string(),
                function: math::mul,
            }),
        );
        env.set(
            "/",
            Value::InternalFunction(InternalFunction {
                name: "/".to_string(),
                function: math::div,
            }),
        );
        env.set(
            "=",
            Value::InternalFunction(InternalFunction {
                name: "=".to_string(),
                function: math::numeric_equal,
            }),
        );

        env
    }

    pub fn eval(&self, input: &str) -> Result<Value, RuntimeError> {
        let token_stream = TokenStream::new(input);
        let mut parser = Parser::new(token_stream);
        let parse_resuilt = parser.parse()?;

        let mut last_result = Value::Void;
        for expr in parse_resuilt {
            last_result = self.evaluator.eval_value(&expr, &self.environment)?;
        }
        Ok(last_result)
    }
}
