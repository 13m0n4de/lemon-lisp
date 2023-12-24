use rug::ops::CompleteRound;
use rug::{Complete, Float, Integer};
use std::str::Chars;

use crate::token::Token;

enum State {
    Normal,
    Escaped,
}

enum TokenizeError {
    UnexpectedChar(char),
    UnclosedString,
}

type TokenResult = Result<Token, TokenizeError>;

struct TokenStream<'a> {
    next_token: Option<TokenResult>,
    char_buffer: Vec<char>,
    char_iter: Chars<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self {
            next_token: None,
            char_buffer: vec![],
            char_iter: s.chars(),
        }
    }

    fn parse_tmp_char(&mut self) -> Option<TokenResult> {
        let tmp_char = String::from_iter(self.char_buffer.drain(..));

        if tmp_char.is_empty() {
            return self.next();
        }

        if let Ok(v) = Integer::parse(&tmp_char) {
            Some(Ok(Token::Integer(v.complete())))
        } else if let Ok(v) = Float::parse(&tmp_char) {
            Some(Ok(Token::Float(v.complete(53))))
        } else {
            Some(Ok(Token::Symbol(tmp_char)))
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.next_token.take() {
            return Some(token);
        }

        match self.char_iter.next() {
            Some(ch) => match ch {
                '(' | '[' => {
                    self.next_token = Some(Ok(Token::LParen));
                    self.parse_tmp_char()
                }
                ')' | ']' => {
                    self.next_token = Some(Ok(Token::RParen));
                    self.parse_tmp_char()
                }
                ' ' | '\n' | '\t' => self.parse_tmp_char(),
                '\'' => {
                    if self.char_buffer.is_empty() {
                        Some(Ok(Token::Quote))
                    } else {
                        Some(Err(TokenizeError::UnexpectedChar('\'')))
                    }
                }
                ';' => {
                    while self.char_iter.next().is_some_and(|c| c != '\n') {}
                    self.parse_tmp_char()
                }
                _ => {
                    if !matches!(ch, '\\' | '{' | '}' | ',' | '`' | '|') {
                        self.char_buffer.push(ch);
                        self.next()
                    } else {
                        Some(Err(TokenizeError::UnexpectedChar(ch)))
                    }
                }
            },
            None => {
                if !self.char_buffer.is_empty() {
                    self.parse_tmp_char()
                } else {
                    None
                }
            }
        }
    }
}
