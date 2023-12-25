use rug::{Complete, Float, Integer};

use super::{ParseError, Token};

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
