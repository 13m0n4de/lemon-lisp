#[cfg(test)]
mod test {
    use lemon_lisp::{
        lexer::TokenStream,
        model::{ParseError, Token, TokenizeError, Value::*},
        parser::Parser,
    };
    use rug::Float;

    macro_rules! test_parser {
        ($name:ident, $($input:expr => $expected:expr),* $(,)?) => {
            #[test]
            fn $name() {
                let test_data = vec![
                    $(($input, $expected)),+
                ];
                for (input, expected) in test_data {
                    let token_stream = TokenStream::new(input);
                    let mut parser = Parser::new(token_stream);
                    assert_eq!(parser.parse(), expected);
                }
            }
        }
    }

    test_parser!(
        test_area_of_a_circle,
        "(define r 10) (define pi 3.14) (* pi (* r r))" => Ok(vec![
            List(vec![
                Symbol("define".into()),
                Symbol("r".into()),
                Integer(10.into()),
            ]),
            List(vec![
                Symbol("define".into()),
                Symbol("pi".into()),
                Float(Float::with_val(53, 3.14)),
            ]),
            List(vec![
                Symbol("*".into()),
                Symbol("pi".into()),
                List(vec![
                    Symbol("*".into()),
                    Symbol("r".into()),
                    Symbol("r".into()),
                ]),
            ]),
        ])
    );

    test_parser!(
        test_quote,
        "(define a '(1 2 3))" => Ok(vec![
            List(vec![
                Symbol("define".into()),
                Symbol("a".into()),
                Quoted(Box::new(
                    List(vec![
                        Integer(1.into()),
                        Integer(2.into()),
                        Integer(3.into()),
                    ])
                )),
            ])
        ]),
        // 调用 quote 函数 和使用 ' 符号不同
        // ' 符号在语法解析阶段就会处理为对应的 Object
        // 而 (quote something) 通过调用函数的方式返回对应的 Object
        r#"(quote (1 2 '3))"# => Ok(vec![
            List(vec![
                Symbol("quote".into()),
                List(vec![
                        Integer(1.into()),
                        Integer(2.into()),
                        Quoted(Box::new(
                            Integer(3.into()),
                        )),
                    ]),
            ])
        ])
    );

    test_parser!(
        test_define_procedure,
        r#"(define (greet name)
             (print (string-append "Hello, " name))
             name)"# => Ok(vec![
            List(vec![
                Symbol("define".into()),
                List(vec![
                    Symbol("greet".into()),
                    Symbol("name".into())
                ]),
                List(vec![
                    Symbol("print".into()),
                    List(vec![
                        Symbol("string-append".into()),
                        String("Hello, ".into()),
                        Symbol("name".into())
                    ])
                ]),
                Symbol("name".into())
            ])
        ])
    );

    test_parser!(
        test_missing_token,
        r#"(print "Hello NAVI""# => Err(ParseError::MissingToken(Token::RParen))
    );

    test_parser!(
        test_invalid_syntax,
        r#"(print "Hello NAVI"))"# => Err(ParseError::InvalidSyntax(Token::RParen)),
    );

    test_parser!(
        test_invalid_digit,
        "#b02" => Err(ParseError::InvalidDigit("02".into())),
        "#d10a" => Err(ParseError::InvalidDigit("10a".into())),
        "#o459" => Err(ParseError::InvalidDigit("459".into())),
        "#xf1g" => Err(ParseError::InvalidDigit("f1g".into())),
    );

    test_parser!(
        test_lexical_error,
        "a|b" => Err(ParseError::LexicalError(TokenizeError::UnexpectedChar('|'))),
        r#" "Hello HAVI "# => Err(ParseError::LexicalError(TokenizeError::UnclosedString)),
    );
}
