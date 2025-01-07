pub mod boolean;
pub mod integer;

use std::fmt::Display;

use crate::error::RuntimeError;

pub trait BelalangType: Display {
    fn type_name(&self) -> &str;
}

pub trait Add<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn add(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait Sub<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn sub(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait Mul<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn mul(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait Div<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn div(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait Mod<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn r#mod(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait Lt<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn lt(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait Le<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn le(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait BitAnd<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_and(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait BitOr<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_or(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait BitXor<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_xor(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait BitSl<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_sl(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
}

pub trait BitSr<Rhs: BelalangType>: BelalangType {
    type Output: BelalangType;
    fn bit_sr(&self, other: &Rhs) -> Result<Self::Output, RuntimeError>;
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
