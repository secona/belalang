use std::fmt::Display;

use belalang_core::token::Token;

use crate::error::RuntimeError;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    pub instructions: Vec<u8>,
    pub arity: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Object {
    #[default]
    Null,
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Function(Function),
    Builtin(usize),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => f.write_str("null"),
            Object::Integer(i) => f.write_str(&format!("{i}")),
            Object::Float(fl) => f.write_str(&format!("{fl}")),
            Object::Boolean(b) => f.write_str(&format!("{b}")),
            Object::String(s) => f.write_str(&s),
            Object::Function(_) => f.write_str("<fn>"),
            Object::Builtin(_) => f.write_str("<builtin fn>"),
        }
    }
}

impl Object {
    pub fn try_add(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l + r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l + r)),

            // different types
            (Self::Integer(i), Self::Float(f)) | (Self::Float(f), Self::Integer(i)) => {
                Ok(Self::Float(i as f64 + f))
            }
            (Self::String(l), r) => Ok(Self::String(format!("{l}{r}"))),
            (l, Self::String(r)) => Ok(Self::String(format!("{l}{r}"))),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Add, r)),
        }
    }

    pub fn try_sub(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l - r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l - r)),

            // different types
            (Self::Integer(l), Self::Float(r)) => Ok(Self::Float(l as f64 - r)),
            (Self::Float(l), Self::Integer(r)) => Ok(Self::Float(l - r as f64)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Sub, r)),
        }
    }

    pub fn try_mul(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l * r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l * r)),

            // strings
            (Self::String(s), Self::Integer(i)) | (Self::Integer(i), Self::String(s)) => {
                Ok(Self::String(s.repeat(i as usize)))
            }

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Mul, r)),
        }
    }

    pub fn try_div(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l / r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l / r)),

            // different types
            (Self::Integer(l), Self::Float(r)) => Ok(Self::Float(l as f64 / r)),
            (Self::Float(l), Self::Integer(r)) => Ok(Self::Float(l / r as f64)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Div, r)),
        }
    }

    pub fn try_mod(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l % r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l % r)),

            // different types
            (Self::Integer(l), Self::Float(r)) => Ok(Self::Float(l as f64 % r)),
            (Self::Float(l), Self::Integer(r)) => Ok(Self::Float(l % r as f64)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Mod, r)),
        }
    }

    pub fn try_less_than(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Boolean(l < r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Boolean(l < r)),
            (Self::String(l), Self::String(r)) => Ok(Self::Boolean(l < r)),

            // different types
            (Self::Integer(l), Self::Float(r)) => Ok(Self::Boolean((l as f64) < r)),
            (Self::Float(l), Self::Integer(r)) => Ok(Self::Boolean(l < (r as f64))),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }

    pub fn try_less_than_equal(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Boolean(l <= r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Boolean(l <= r)),
            (Self::String(l), Self::String(r)) => Ok(Self::Boolean(l <= r)),

            // different types
            (Self::Integer(l), Self::Float(r)) => Ok(Self::Boolean((l as f64) <= r)),
            (Self::Float(l), Self::Integer(r)) => Ok(Self::Boolean(l <= (r as f64))),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }
}
