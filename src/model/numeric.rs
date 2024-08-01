use core::fmt;
use std::ops::{Add, Div, Mul, Sub};

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
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a + b),
            (Self::Integer(a), Self::Float(b)) => Self::Float(a + b),
            (Self::Float(a), Self::Integer(b)) => Self::Float(a + b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a + b),
        }
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a - b),
            (Self::Integer(a), Self::Float(b)) => Self::Float(a - b),
            (Self::Float(a), Self::Integer(b)) => Self::Float(a - b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a - b),
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a * b),
            (Self::Integer(a), Self::Float(b)) => Self::Float(a * b),
            (Self::Float(a), Self::Integer(b)) => Self::Float(a * b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a * b),
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a / b),
            (Self::Integer(a), Self::Float(b)) => Self::Float(a / b),
            (Self::Float(a), Self::Integer(b)) => Self::Float(a / b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a / b),
        }
    }
}
