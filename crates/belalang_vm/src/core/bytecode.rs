//! Bytecode used by The Belalang VM.
//!
//! # Note
//! The structure of this module may change, or even removed. There is no need
//! for a separate module specifically for Bytecode definitions. I plan to move
//! this to [`crate::vm`], but not yet decided.

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Constant {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
    String(&'static str),
}

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Constant>,
}
