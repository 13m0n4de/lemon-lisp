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
