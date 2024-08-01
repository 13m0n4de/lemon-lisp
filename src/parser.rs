use crate::{
    lexer::LexResult,
    model::{ParseError, Token, Value},
};
use std::iter::Peekable;

pub struct Parser<I>
where
    I: Iterator<Item = LexResult>,
{
    lexer: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = LexResult>,
{
    pub fn new(lexer: I) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn eat(&mut self, expected: &Token) -> Result<(), ParseError> {
        if !self.peek_token_is(expected)? {
            let Some(Ok(current_token)) = self.lexer.next() else {
                unreachable!()
            };
            Err(ParseError::UnexpectedToken {
                expected: expected.clone(),
                found: current_token,
            })
        } else {
            self.lexer.next();
            Ok(())
        }
    }

    fn peek_token_is(&mut self, expected: &Token) -> Result<bool, ParseError> {
        let next = self
            .lexer
            .peek()
            .ok_or(ParseError::MissingToken(expected.clone()))?;

        match next {
            Ok(token) => Ok(token == expected),
            Err(err) => Err(ParseError::LexicalError(err.clone())),
        }
    }

    fn parse_quoted(&mut self) -> Result<Value, ParseError> {
        let value = self.parse_atom()?.ok_or(ParseError::UnexpectedEOF)?;
        Ok(Value::Quoted(Box::new(value)))
    }

    fn parse_list(&mut self) -> Result<Value, ParseError> {
        self.eat(&Token::LParen)?;
        let mut list: Vec<Value> = vec![];

        while !self.peek_token_is(&Token::RParen)? {
            if let Some(value) = self.parse_atom()? {
                list.push(value);
            }
        }

        self.eat(&Token::RParen)?;
        Ok(Value::List(list))
    }

    fn parse_atom(&mut self) -> Result<Option<Value>, ParseError> {
        if let Some(next) = self.lexer.peek() {
            match next.clone()? {
                Token::LParen => Ok(Some(self.parse_list()?)),
                Token::RParen => Err(ParseError::InvalidSyntax(Token::RParen)),
                Token::Quote => {
                    self.lexer.next();
                    Ok(Some(self.parse_quoted()?))
                }
                token => {
                    self.lexer.next();
                    Ok(Some(token.try_into()?))
                }
            }
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
