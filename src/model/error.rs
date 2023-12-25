use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenizeError {
    UnexpectedChar(char),
    UnclosedString,
}

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
