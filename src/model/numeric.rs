use core::fmt;
use std::ops::Add;

use rug::{Float, Integer};

#[derive(Debug, PartialEq, Clone)]
pub enum Numeric {
    Integer(Integer),
    Float(Float),
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(n) => write!(f, "{}", n),
            Self::Float(n) => write!(f, "{}", n),
        }
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
