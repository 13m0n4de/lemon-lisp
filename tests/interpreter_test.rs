#[cfg(test)]
mod tests {
    use lemon_lisp::{interpreter::Interpreter, model::Value};
    use rug::{Float, Integer};

    #[test]
    fn test_simple_arithmetic() {
        let interpreter = Interpreter::new();

        assert_eq!(interpreter.eval("(+ 4)"), Ok(Value::from(Integer::from(4))));
        assert_eq!(
            interpreter.eval("(+ 1 2 3)"),
            Ok(Value::from(Integer::from(6)))
        );

        assert_eq!(
            interpreter.eval("(- 4)"),
            Ok(Value::from(Integer::from(-4)))
        );
        assert_eq!(
            interpreter.eval("(- 9 5 2)"),
            Ok(Value::from(Integer::from(2)))
        );

        assert_eq!(interpreter.eval("(* 7)"), Ok(Value::from(Integer::from(7))));
        assert_eq!(
            interpreter.eval("(* 1 2 3)"),
            Ok(Value::from(Integer::from(6)))
        );

        assert_eq!(
            interpreter.eval("(/ 4)"),
            Ok(Value::from(Float::with_val(53, 0.25)))
        );
        assert_eq!(
            interpreter.eval("(/ 45 5 3)"),
            Ok(Value::from(Integer::from(3)))
        );
    }

    #[test]
    fn test_nested_arithmetic() {
        let interpreter = Interpreter::new();

        assert_eq!(
            interpreter.eval("(+ 1 (* 2 3) (- 10 5))"),
            Ok(Value::from(Integer::from(12)))
        );

        assert_eq!(
            interpreter.eval("(+ (* 2 (+ 3 4)) (- 10 5))"),
            Ok(Value::from(Integer::from(19)))
        );

        assert_eq!(
            interpreter.eval("(* (/ 10 2) (- 7 3))"),
            Ok(Value::from(Integer::from(20)))
        );

        assert_eq!(
            interpreter.eval("(- (+ 5 (* 3 4)) (/ 20 (- 6 2)))"),
            Ok(Value::from(Integer::from(12)))
        );

        assert_eq!(
            interpreter.eval("(* (+ 2.5 1.5) (- 10 6))"),
            Ok(Value::from(Float::with_val(53, 16.0)))
        );

        assert_eq!(
            interpreter.eval("(+ 1 (- 2 (* 3 (/ 12 (+ 2 2)))))"),
            Ok(Value::from(Integer::from(-6)))
        );

        assert_eq!(
            interpreter.eval("(* (- 0 3) (+ 4 (/ 10 5)))"),
            Ok(Value::from(Integer::from(-18)))
        );

        assert_eq!(
            interpreter.eval("(- (* 2.5 (+ 3 4)) (/ 15 3))"),
            Ok(Value::from(Float::with_val(53, 12.5)))
        );
    }
}
