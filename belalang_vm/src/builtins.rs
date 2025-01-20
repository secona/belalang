use crate::types::integer::BelalangInteger;
use crate::types::BelalangType;

pub type BuiltinFn = fn(&[Box<dyn BelalangType>]) -> Box<dyn BelalangType>;

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
