use core::fmt;
use rug::{Complete, Float, Integer};

use crate::internal::InternalFunction;

use super::{Closure, Keyword, Numeric, ParseError, RuntimeError, TailRecursiveClosure, Token};

/// 包含了所有可能的 Lisp 值，包括原子、列表等等。
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Void,
    Numeric(Numeric),
    Bool(bool),
    Symbol(String),
    String(String),
    List(Vec<Value>),
    Quoted(Box<Value>),
    Keyword(Keyword),
    Closure(Closure),
    TailRecursiveClosure(TailRecursiveClosure),
    InternalFunction(InternalFunction),
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
    ///     Ok(Value::from(rug::Integer::from(123))),
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

            Token::Integer(i) => Ok(i.into()),
            Token::Float(f) => Ok(f.into()),
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

                    Ok(value.into())
                }
                "define" => Ok(Value::Keyword(Keyword::Define)),
                "lambda" => Ok(Value::Keyword(Keyword::Lambda)),
                _ => Ok(Value::Symbol(symbol)),
            },
        }
    }
}

impl From<Integer> for Value {
    fn from(value: Integer) -> Self {
        Value::Numeric(Numeric::Integer(value))
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Value::Numeric(Numeric::Float(value))
    }
}

impl From<Numeric> for Value {
    fn from(value: Numeric) -> Self {
        Value::Numeric(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void => write!(f, "#<void>"),
            Value::Numeric(n) => write!(f, "{}", n),
            Value::Bool(bool) => match bool {
                true => write!(f, "#t"),
                false => write!(f, "#f"),
            },
            Value::Symbol(symbol) => write!(f, "{}", symbol),
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::List(list) => {
                write!(
                    f,
                    "({})",
                    list.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Value::Quoted(value) => write!(f, "'{}", value),
            Value::Keyword(keyword) => write!(f, "#<keyword:{}>", keyword),
            Value::Closure(lambda) => match &lambda.name {
                Some(name) => write!(f, "#<procedure:{}>", name),
                None => write!(f, "#<procedure>"),
            },
            Value::TailRecursiveClosure(tail_recursive_closure) => {
                match &tail_recursive_closure.closure.name {
                    Some(name) => write!(f, "#<procedure:{}>", name),
                    None => write!(f, "#<procedure>"),
                }
            }
            Value::InternalFunction(internal_function) => {
                write!(f, "#<procedure:{}>", internal_function.name)
            }
        }
    }
}

macro_rules! try_as_type {
    ( $( $name:ident; $variant:pat => $result:expr; $ty:ty; $expected:expr ),* $(,)? ) => {
        $(
            pub fn $name(&self) -> Result<$ty, RuntimeError> {
                match self {
                    $variant => $result,
                    _ => Err(RuntimeError::TypeError {
                        expected: $expected,
                        founded: self.clone()
                    }),
                }
            }
        )*
    }
}

impl Value {
    try_as_type! {
        try_as_bool; Value::Bool(b) => Ok(*b); bool; "bool",
        try_as_numeric; Value::Numeric(n) => Ok(n.clone()); Numeric; "numeric",
        try_as_symbol; Value::Symbol(s) => Ok(s); &String; "symbol",
        try_as_list; Value::List(l) => Ok(l); &Vec<Value>; "list",
    }
}
