#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("stack underflow")]
    StackUnderflow,

    #[error("unknown instruction: {0}")]
    UnknownInstruction(u8),

    #[error("attempt to call non-function")]
    NotAFunction,
}
