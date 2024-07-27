use std::fmt::Display;

use belalang_core::token::Token;

use crate::error::RuntimeError;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    pub instructions: Vec<u8>,
    pub arity: usize,
    pub pointer: usize,
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
    Array(Vec<Object>),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(i) => write!(f, "{i}"),
            Object::Float(fl) => write!(f, "{fl}"),
            Object::Boolean(b) => write!(f, "{b}"),
            Object::String(s) => write!(f, r#""{s}""#),
            Object::Function(_) => write!(f, "<fn>"),
            Object::Builtin(_) => write!(f, "<builtin fn>"),
            Object::Array(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
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

    pub fn try_bit_and(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l & r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }

    pub fn try_bit_or(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l | r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }

    pub fn try_bit_xor(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l ^ r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }

    pub fn try_bit_sl(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l << r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }

    pub fn try_bit_sr(self, rhs: Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            // same type
            (Self::Integer(l), Self::Integer(r)) => Ok(Self::Integer(l >> r)),

            // unsupported
            (l, r) => Err(RuntimeError::InvalidOperation(l, Token::Lt, r)),
        }
    }
}
