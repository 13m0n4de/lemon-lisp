use crate::{
    lexer::TokenResult,
    model::{ParseError, Value},
};
use std::iter::Peekable;

pub struct Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    lexer: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    pub fn new(lexer: I) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn parse_atom(&mut self) -> Result<Option<Value>, ParseError> {
        if let Some(next) = self.lexer.peek() {
            todo!()
        } else {
            Ok(None)
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Value>, ParseError> {
        let mut list: Vec<Value> = vec![];
        while let Some(value) = self.parse_atom()? {
            list.push(value);
        }
        Ok(list)
    }
}
