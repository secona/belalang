mod core;
pub mod errors;
pub mod functions;
pub mod mem;
pub mod objects;
mod ptr;

pub use core::VM;
pub(crate) use core::with_heap;

pub use ptr::BelalangPtr;
