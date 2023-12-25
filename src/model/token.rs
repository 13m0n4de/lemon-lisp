use rug::{Float, Integer};

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(Integer),
    Float(Float),
    String(String),
    Quote,
}
