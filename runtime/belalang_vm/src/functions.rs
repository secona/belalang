//! Functions provided by The Belalang VM.

use crate::objects::BelalangObject;

mod print;

pub use print::*;

pub type BuiltinFn = fn(&[Box<dyn BelalangObject>]) -> Box<dyn BelalangObject>;

pub static BUILTIN_FUNCTIONS: &[(&str, BuiltinFn)] = &[("print", belalang_print)];

pub fn lookup_builtin(name: &str) -> Option<BuiltinFn> {
    BUILTIN_FUNCTIONS
        .binary_search_by_key(&name, |&(n, _)| n)
        .ok()
        .map(|idx| BUILTIN_FUNCTIONS[idx].1)
}
