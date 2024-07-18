#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("stack underflow")]
    StackUnderflow,

    #[error("unknown instruction: {0}")]
    UnknownInstruction(u8),

    #[error("unknown builtin function")]
    UnknownBuiltinFunction,

    #[error("attempt to call non-function")]
    NotAFunction,
}
