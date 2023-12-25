use rug::{Float, Integer};

pub enum Value {
    Void,
    Integer(Integer),
    Float(Float),
    Bool(bool),
    Symbol(String),
    List(Vec<Value>),
    Quoted(Box<Value>),
}
