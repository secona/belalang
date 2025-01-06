use crate::object::Object;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("stack underflow")]
    StackUnderflow,

    #[error("stack overflow")]
    StackOverflow,

    #[error("unknown instruction: {0}")]
    UnknownInstruction(u8),

    #[error("unknown builtin function")]
    UnknownBuiltinFunction,

    #[error("invalid operation: {0} {1} {2}")]
    InvalidOperation(Object, String, Object),

    #[error("attempt to call non-function")]
    NotAFunction,
}
