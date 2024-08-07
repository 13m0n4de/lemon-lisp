use rug::ops::CompleteRound;
use rug::{Complete, Float, Integer};
use std::str::Chars;

use crate::model::{Token, TokenizeError};

// 字符串解析状态，普通或者转义
enum State {
    Normal,
    Escaped,
}

/// 词法解析结果
pub type LexResult = Result<Token, TokenizeError>;

/// 标记流
pub struct TokenStream<'a> {
    // 下一个 Token，用于在迭代中预先设置下一次的结果
    pending_token: Option<LexResult>,
    // 字符缓冲区
    char_buffer: String,
    // 字符迭代器
    input_chars: Chars<'a>,
}

impl<'a> TokenStream<'a> {
    /// 创建标记流
    /// ```rust
    /// # use lemon_lisp::lexer::TokenStream;
    /// #
    /// let expression = "(+ 1 2)";
    /// let token_stream = TokenStream::new(expression);
    /// ```
    pub fn new(s: &'a str) -> Self {
        Self {
            pending_token: None,
            char_buffer: String::new(),
            input_chars: s.chars(),
        }
    }

    fn parse_buffered_char(&mut self) -> Option<LexResult> {
        let token_str = std::mem::take(&mut self.char_buffer);

        if token_str.is_empty() {
            return self.next();
        }

        // 如果不是数字这解析为 Symbol
        if let Ok(v) = Integer::parse(&token_str) {
            Some(Ok(Token::Integer(v.complete())))
        } else if let Ok(v) = Float::parse(&token_str) {
            Some(Ok(Token::Float(v.complete(53))))
        } else {
            Some(Ok(Token::Symbol(token_str)))
        }
    }

    fn parse_string(&mut self) -> LexResult {
        let mut state = State::Normal;
        let mut string_content = String::new();

        for ch in self.input_chars.by_ref() {
            match state {
                State::Normal => match ch {
                    '\\' => state = State::Escaped,                  // 进入转义状态
                    '"' => return Ok(Token::String(string_content)), // 字符串结束
                    _ => string_content.push(ch),                    // 添加字符到缓存中
                },
                State::Escaped => {
                    string_content.push(match ch {
                        '"' => '"',
                        'n' => '\n',
                        _ => ch,
                    });
                    state = State::Normal // 转义完毕返回正常状态
                }
            }
        }
        // 字符串未闭合，正常应该在之前的循环中 return
        Err(TokenizeError::UnclosedString)
    }

    /// 进行词法解析，返回标记列表
    /// ```rust
    /// # use lemon_lisp::{
    /// #     lexer::TokenStream,
    /// #     model::Token
    /// # };
    /// #
    /// let token_stream = TokenStream::new("(+ 1)");
    ///
    /// assert_eq!(
    ///     Ok(vec![
    ///         Token::LParen,
    ///         Token::Symbol("+".into()),
    ///         Token::Integer(1.into()),
    ///         Token::RParen
    ///     ]),
    ///     token_stream.tokenize()
    /// );
    /// ```
    pub fn tokenize(mut self) -> Result<Vec<Token>, TokenizeError> {
        self.try_collect()
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.pending_token.take() {
            return Some(token);
        }

        match self.input_chars.next() {
            Some(ch) => self.process_char(ch),
            // 没有字符的时候检查缓冲区是否还有内容
            None => {
                if !self.char_buffer.is_empty() {
                    self.parse_buffered_char()
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> TokenStream<'a> {
    fn process_char(&mut self, ch: char) -> Option<LexResult> {
        match ch {
            // 左右括号
            // 对应设置下一个 Token
            // 解析缓冲区内容
            '(' | '[' => {
                self.pending_token = Some(Ok(Token::LParen));
                self.parse_buffered_char()
            }
            ')' | ']' => {
                self.pending_token = Some(Ok(Token::RParen));
                self.parse_buffered_char()
            }

            // 遇到空白字符开始解析缓冲区中的内容
            ' ' | '\n' | '\t' => self.parse_buffered_char(),

            // 开始解析字符串
            // 如果缓冲区为空，表示字符串解析成功
            // 否则解析成功，但存在多余字符
            '"' => {
                let result = self.parse_string();
                if self.char_buffer.is_empty() {
                    Some(result)
                } else {
                    self.pending_token = Some(result);
                    Some(Err(TokenizeError::UnexpectedChar('"')))
                }
            }

            // 引用
            // 当缓冲区为空时解析为 Quote
            // 不为空时说明在符号中间插入了单引号，不合语法
            '\'' => {
                if self.char_buffer.is_empty() {
                    Some(Ok(Token::Quote))
                } else {
                    Some(Err(TokenizeError::UnexpectedChar('\'')))
                }
            }

            // 注释符忽略此行
            ';' => {
                while self.input_chars.next().is_some_and(|c| c != '\n') {}
                self.parse_buffered_char()
            }

            // 不是非法字符就加入缓冲区
            _ => {
                if !matches!(ch, '\\' | '{' | '}' | ',' | '`' | '|') {
                    self.char_buffer.push(ch);
                    self.next()
                } else {
                    Some(Err(TokenizeError::UnexpectedChar(ch)))
                }
            }
        }
    }
}
