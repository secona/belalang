mod core;
pub mod errors;
pub mod functions;
pub mod mem;
pub mod objects;
pub mod prelude;
mod ptr;

pub use core::VM;
pub(crate) use core::with_heap;

pub(crate) use prelude::*;
pub use ptr::BelalangPtr;
