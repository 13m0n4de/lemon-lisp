use core::fmt;

use rug::{Float, Integer};

/// 词法分析器 [`crate::lexer`] 中所有的标记
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(Integer),
    Float(Float),
    String(String),
    Quote,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::Integer(integer) => write!(f, "{}", integer),
            Token::Float(float) => write!(f, "{}", float),
            Token::String(string) => write!(f, "\"{}\"", string),
            Token::Quote => write!(f, "'"),
        }
    }
}
