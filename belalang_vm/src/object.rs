use std::fmt::Display;

use crate::error::RuntimeError;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    pub arity: usize,
    pub pointer: usize,
    pub locals_count: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Object {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(i) => write!(f, "{i}"),
            Object::Boolean(b) => write!(f, "{b}"),
        }
    }
}

impl Object {
    pub fn try_add(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l + r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "+".into(), r)),
        }
    }

    pub fn try_sub(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l - r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "-".into(), r)),
        }
    }

    pub fn try_mul(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l * r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "*".into(), r)),
        }
    }

    pub fn try_div(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l / r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "/".into(), r)),
        }
    }

    pub fn try_mod(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l % r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "%".into(), r)),
        }
    }

    pub fn try_less_than(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Boolean(l < r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "<".into(), r)),
        }
    }

    pub fn try_less_than_equal(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Boolean(l <= r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "<=".into(), r)),
        }
    }

    pub fn try_bit_and(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l & r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "&".into(), r)),
        }
    }

    pub fn try_bit_or(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l | r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "|".into(), r)),
        }
    }

    pub fn try_bit_xor(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l ^ r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "^".into(), r)),
        }
    }

    pub fn try_bit_sl(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l << r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, "<<".into(), r)),
        }
    }

    pub fn try_bit_sr(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l >> r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, ">>".into(), r)),
        }
    }
}
