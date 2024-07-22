use belalang_core::token::Token;

use crate::object::Object;

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("stack underflow")]
    StackUnderflow,

    #[error("unknown instruction: {0}")]
    UnknownInstruction(u8),

    #[error("unknown builtin function")]
    UnknownBuiltinFunction,

    #[error("invalid operation: {0} {1} {2}")]
    InvalidOperation(Object, Token, Object),

    #[error("attempt to call non-function")]
    NotAFunction,
}
