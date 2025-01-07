use crate::object::Object;

pub type BuiltinFn = fn(&[Object]) -> Object;

pub static BUILTIN_FUNCTIONS: &[(&str, BuiltinFn)] = &[("print", |args| {
    println!("{}", args.first().unwrap());
    Object::Null
})];

pub fn lookup_builtin(name: &str) -> Option<BuiltinFn> {
    BUILTIN_FUNCTIONS
        .binary_search_by_key(&name, |&(n, _)| n)
        .ok()
        .map(|idx| BUILTIN_FUNCTIONS[idx].1)
}
