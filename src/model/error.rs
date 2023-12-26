use super::Token;

/// 词法分析中可能发生的错误
#[derive(Debug, PartialEq, Clone)]
pub enum TokenizeError {
    UnexpectedChar(char),
    UnclosedString,
}

/// 语法分析中可能发生的错误
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    UnexpectedToken { expected: Token, found: Token },
    MissingToken(Token),
    InvalidSyntax(Token),
    InvalidDigit(String),
    LexicalError(TokenizeError),
    NonConvertibleToken(Token),
    UnexpectedEOF,
}

impl From<TokenizeError> for ParseError {
    fn from(value: TokenizeError) -> Self {
        ParseError::LexicalError(value)
    }
}
