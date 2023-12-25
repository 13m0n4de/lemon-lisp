use super::Token;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    UnexpectedChar(char),
    UnclosedString,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    MissingToken(&'static str),
    InvalidSyntax(Token),
    InvalidDigit(String),
    LexicalError(TokenizeError),
    NonConvertibleToken(Token),
    UnexpectedEOF,
}
