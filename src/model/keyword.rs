use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Define,
    Lambda,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Define => write!(f, "define"),
            Keyword::Lambda => write!(f, "lambda"),
        }
    }
}
