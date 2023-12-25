use rug::{Float, Integer};

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
