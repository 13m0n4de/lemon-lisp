use rug::{Integer, Float};

pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(Integer),
    Float(Float),
    String(String),
    Quote
}
