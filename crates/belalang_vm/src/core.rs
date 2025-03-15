//! Core implementations of Belalang VM.
//!
//! This module includes everything that is needed to run the Belalang VM, from
//! the VM itself, opcodes, and bytecode used.

pub mod bytecode;
pub mod opcode;
mod vm;

pub use vm::VM;
