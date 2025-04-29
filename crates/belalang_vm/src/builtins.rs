//! Built-ins provided by The Belalang VM.
//!
//! # Note
//! This structure may change to accommodate for different types of builtins,
//! such as functions, types, and objects. Currently, objects live on their own
//! module, [`crate::objects`].

use crate::objects::BelalangObject;
use crate::objects::integer::BelalangInteger;

pub type BuiltinFn = fn(&[Box<dyn BelalangObject>]) -> Box<dyn BelalangObject>;

pub static BUILTIN_FUNCTIONS: &[(&str, BuiltinFn)] = &[("print", |args| {
    // TODO: handle null values

    println!("{}", args.first().unwrap());
    Box::new(BelalangInteger::new(0))
})];

pub fn lookup_builtin(name: &str) -> Option<BuiltinFn> {
    BUILTIN_FUNCTIONS
        .binary_search_by_key(&name, |&(n, _)| n)
        .ok()
        .map(|idx| BUILTIN_FUNCTIONS[idx].1)
}
