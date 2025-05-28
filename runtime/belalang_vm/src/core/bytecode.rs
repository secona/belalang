//! Bytecode used by The Belalang VM.
//!
//! Defines the structure and components used to represent compiled bytecode,
//! including instructions and constants.

/// Constants used in the Belalang bytecode
///
/// These values are used to represent literal data embedded in the bytecode.
/// They are referenced by index in the constant pool during execution.
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Constant {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
    String(&'static str),
}

/// A compiled bytecode object for the Belalang VM
///
/// This contains the instruction stream and associated constant pool needed for
/// execution by the virtual machine.
pub struct Bytecode {
    /// The instructions to be executed
    ///
    /// Each byte corresponds to an opcode defined in [crate::core::opcode].
    pub instructions: Vec<u8>,

    /// Constant values referenced by the bytecode
    pub constants: Vec<Constant>,
}
