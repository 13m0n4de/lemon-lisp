use rug::{Complete, Float, Integer};

use super::{ParseError, Token};

/// [`Value`] 枚举包含了所有可能的 Lisp 值，包括原子、列表等等。
#[derive(Debug, PartialEq)]
pub enum Value {
    Void,
    Integer(Integer),
    Float(Float),
    Bool(bool),
    Symbol(String),
    String(String),
    List(Vec<Value>),
    Quoted(Box<Value>),
}

impl TryFrom<Token> for Value {
    type Error = ParseError;

    /// 尝试将 [`Token`] 转换为 [`Value`]。
    ///
    /// # 示例
    /// ## 将整数 Token 转换为 Value
    ///
    /// ```rust
    /// # use rug;
    /// # use lemon_lisp::model::{Token, Value};
    /// #
    /// let integer_token = Token::Integer(rug::Integer::from(123));
    /// 
    /// assert_eq!(
    ///     Ok(Value::Integer(rug::Integer::from(123))),
    ///     Value::try_from(integer_token),
    /// );
    /// ```
    ///
    /// ## 将符号 Token 转换为 Value
    ///
    /// ```rust
    /// # use lemon_lisp::model::{Token, Value};
    /// #
    /// let symbol_token = Token::Symbol("#t".to_string());
    ///
    /// assert_eq!(
    ///     Ok(Value::Bool(true)),
    ///     Value::try_from(symbol_token)
    /// );
    /// ```
    ///
    /// # 错误
    ///
    /// 当遇到错误语法或不可转换的 [`Token`] 时，会返回 [`ParseError`] 。
    ///
    /// ```rust
    /// # use lemon_lisp::model::{Token, Value, ParseError};
    /// #
    /// let invalid_token = Token::RParen;
    ///
    /// assert_eq!(
    ///     Err(ParseError::NonConvertibleToken(Token::RParen)),
    ///     Value::try_from(invalid_token)
    /// );
    /// ```
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::LParen | Token::RParen | Token::Quote => {
                Err(ParseError::NonConvertibleToken(token))
            }

            Token::Integer(i) => Ok(Value::Integer(i)),
            Token::Float(f) => Ok(Value::Float(f)),
            Token::String(s) => Ok(Value::String(s)),

            Token::Symbol(symbol) => match symbol.as_str() {
                "#t" => Ok(Value::Bool(true)),
                "#f" => Ok(Value::Bool(false)),
                s if s.starts_with('#') => {
                    let radix = match s.chars().nth(1) {
                        Some('b') => 2,
                        Some('o') => 8,
                        Some('d') => 10,
                        Some('x') => 16,
                        _ => return Err(ParseError::InvalidSyntax(Token::Symbol(symbol))),
                    };
                    let digits = &s[2..];
                    let value = Integer::parse_radix(digits, radix)
                        .map_err(|_| ParseError::InvalidDigit(digits.into()))?
                        .complete();

                    Ok(Value::Integer(value))
                }
                _ => Ok(Value::Symbol(symbol)),
            },
        }
    }
}
