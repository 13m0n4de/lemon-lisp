use rug::ops::CompleteRound;
use rug::{Complete, Float, Integer};
use std::str::Chars;

use crate::model::{Token, TokenizeError};

enum State {
    Normal,
    Escaped,
}

pub type TokenResult = Result<Token, TokenizeError>;

pub struct TokenStream<'a> {
    next_token: Option<TokenResult>,
    char_buffer: Vec<char>,
    char_iter: Chars<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn new(s: &'a str) -> Self {
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

    fn parse_string(&mut self) -> TokenResult {
        let mut state = State::Normal;
        let mut tmp_str = String::new();

        for ch in self.char_iter.by_ref() {
            match state {
                State::Normal => match ch {
                    '\\' => state = State::Escaped,
                    '"' => return Ok(Token::String(tmp_str)),
                    _ => tmp_str.push(ch),
                },
                State::Escaped => {
                    match ch {
                        '"' => tmp_str.push('"'),
                        'n' => tmp_str.push('\n'),
                        _ => tmp_str.push(ch),
                    }
                    state = State::Normal
                }
            }
        }
        Err(TokenizeError::UnclosedString)
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, TokenizeError> {
        self.try_collect()
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
                '"' => {
                    let result = self.parse_string();
                    if self.char_buffer.is_empty() {
                        Some(result)
                    } else {
                        self.next_token = Some(result);
                        Some(Err(TokenizeError::UnexpectedChar('"')))
                    }
                }
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
