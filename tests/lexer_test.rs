#[cfg(test)]
mod tests {
    use lemon_lisp::{
        lexer::TokenStream,
        model::{Token::*, TokenizeError},
    };
    use rug::Float;

    macro_rules! test_lexer {
        ($name:ident, $($input:expr => $expected:expr),* $(,)?) => {
            #[test]
            fn $name() {
                let test_data = vec![
                    $(($input, $expected)),+
                ];
                for (input, expected) in test_data {
                    let token_stream = TokenStream::new(input);
                    assert_eq!(token_stream.tokenize(), expected);
                }
            }
        }
    }

    test_lexer! {
        test_integer,
        "1" => Ok(vec![Integer(1.into())]),
        "-2" => Ok(vec![Integer((-2).into())]),
        "+5" => Ok(vec![Integer(5.into())]),
    }

    test_lexer! {
        test_float,
        "1.2" => Ok(vec![Float(Float::with_val(53, 1.2))]),
        ".05" => Ok(vec![Float(Float::with_val(53, 0.05))]),
        "-3.1" => Ok(vec![Float(Float::with_val(53, -3.1))]),
        "+2.5" => Ok(vec![Float(Float::with_val(53, 2.5))]),
    }

    test_lexer!(
        test_string,
        r#" "Hello NAVI" "# => Ok(vec![String("Hello NAVI".into())])
    );

    test_lexer!(
        test_esccaped_string,
        r#" "Hello \"Lain\"" "# => Ok(vec![String(r#"Hello "Lain""#.into())])
    );

    test_lexer!(
        test_quoted,
        "'a" => Ok(vec![Quote, Symbol("a".into())]),
        "(quote x)" => Ok(vec![LParen, Symbol("quote".into()), Symbol("x".into()), RParen]),
    );

    test_lexer!(
        test_area_of_a_circle,
        "(define r 10) (define pi 3.14) (* pi (* r r))" =>  Ok(vec![
            LParen, Symbol("define".into()), Symbol("r".into()), Integer(10.into()), RParen,
            LParen, Symbol("define".into()), Symbol("pi".into()), Float(Float::with_val(53, 3.140)), RParen,
            LParen, Symbol("*".into()), Symbol("pi".into()),
                LParen, Symbol("*".into()), Symbol("r".into()), Symbol("r".into()), RParen,
            RParen
        ])
    );

    test_lexer!(
        test_comment,
        r#"(define (square n)
             ; A semi-colon starts a line comment.
             ; The expression below is the function body.
             (filled-rectangle n n))"# => Ok(vec![
            LParen, Symbol("define".into()),
                LParen, Symbol("square".into()), Symbol("n".into()), RParen,
                LParen, Symbol("filled-rectangle".into()), Symbol("n".into()), Symbol("n".into()), RParen,
            RParen,
        ])
    );

    test_lexer!(
        test_unexpected_char,
        "(let ([x 1] {y 2.3}) (+ x y))" => Err(TokenizeError::UnexpectedChar('{')),
        "(define a,b 2)" => Err(TokenizeError::UnexpectedChar(',')),
        "(define `a 2)" => Err(TokenizeError::UnexpectedChar('`')),
        "(define a|b 3)" => Err(TokenizeError::UnexpectedChar('|')),
    );

    test_lexer!(
        test_unclosed_string,
        r#" "Hello "# => Err(TokenizeError::UnclosedString),
        r#" "Hello" "NAVI "# => Err(TokenizeError::UnclosedString),
        r#" "Hello, NAVI\" "# => Err(TokenizeError::UnclosedString),
        r#" "Hello
             NAVI
        "# => Err(TokenizeError::UnclosedString),
    );
}
