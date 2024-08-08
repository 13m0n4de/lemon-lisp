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

macro_rules! impl_numeric_op {
    ($trait:ident, $method:ident) => {
        impl $trait for Numeric {
            type Output = Numeric;

            fn $method(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Self::Integer(a), Self::Integer(b)) => Self::Integer(a.$method(b)),
                    (Self::Integer(a), Self::Float(b)) => Self::Float(a.$method(b)),
                    (Self::Float(a), Self::Integer(b)) => Self::Float(a.$method(b)),
                    (Self::Float(a), Self::Float(b)) => Self::Float(a.$method(b)),
                }
            }
        }
    };
}

impl_numeric_op!(Add, add);
impl_numeric_op!(Sub, sub);
impl_numeric_op!(Mul, mul);
impl_numeric_op!(Div, div);

impl Numeric {
    pub fn is_zero(&self) -> bool {
        match self {
            Numeric::Integer(n) => n.is_zero(),
            Numeric::Float(f) => f.is_zero(),
        }
    }
}
