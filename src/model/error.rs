use core::fmt;

use super::{Token, Value};

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

/// 解释执行中可能发生的错误
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeError {
        expected: &'static str,
        founded: Value,
    },
    OperationError {
        operation: &'static str,
        lhs_type: &'static str,
        rhs_type: &'static str,
    },
    InvalidListLength {
        expected: usize,
        founded: usize,
    },
    InvalidArity {
        expected: usize,
        founded: usize,
    },
    DivideByZero,
    NonCallableValue(Value),
    EmptyList,
    SyntaxError(ParseError),
    InvalidClosure,
}

impl From<TokenizeError> for ParseError {
    fn from(value: TokenizeError) -> Self {
        ParseError::LexicalError(value)
    }
}

impl From<ParseError> for RuntimeError {
    fn from(value: ParseError) -> Self {
        RuntimeError::SyntaxError(value)
    }
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizeError::UnexpectedChar(c) => {
                write!(f, "Unexpected character: {}", c)
            }
            TokenizeError::UnclosedString => {
                write!(f, "Unclosed string")
            }
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token: expected {}, found {}",
                    expected, found
                )
            }
            ParseError::MissingToken(token_type) => {
                write!(f, "Missing token: {}", token_type)
            }
            ParseError::InvalidSyntax(token) => {
                write!(f, "Invalid syntax: {}", token)
            }
            ParseError::InvalidDigit(digit) => {
                write!(f, "Invalid digit: {}", digit)
            }
            ParseError::LexicalError(error) => {
                write!(f, "Lexical error: {}", error)
            }
            ParseError::NonConvertibleToken(token) => {
                write!(f, "Non-convertible token: {}", token)
            }
            ParseError::UnexpectedEOF => {
                write!(f, "Unexpected EOF")
            }
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            RuntimeError::UndefinedFunction(name) => {
                write!(f, "Undefined function: {}", name)
            }
            RuntimeError::TypeError { expected, founded } => {
                write!(f, "TypeError: expected {}, found {}", expected, founded)
            }
            RuntimeError::OperationError {
                operation,
                lhs_type,
                rhs_type,
            } => {
                write!(
                    f,
                    "OperationError: {} between {} and {}",
                    operation, lhs_type, rhs_type
                )
            }
            RuntimeError::InvalidListLength { expected, founded } => {
                write!(
                    f,
                    "InvalidListLength: expected {}, found {}",
                    expected, founded
                )
            }
            RuntimeError::InvalidArity { expected, founded } => {
                write!(
                    f,
                    "Invalid arity: expected {} arguments, but found {}",
                    expected, founded
                )
            }
            RuntimeError::DivideByZero => {
                write!(f, "DivideByZero")
            }
            RuntimeError::NonCallableValue(value) => {
                write!(f, "NonCallableValue: {}", value)
            }
            RuntimeError::EmptyList => {
                write!(f, "EmptyList")
            }
            RuntimeError::SyntaxError(parse_error) => {
                write!(f, "SyntaxError: {}", parse_error)
            }
            RuntimeError::InvalidClosure => write!(f, "InvalidClosure"),
        }
    }
}
