use super::object::Object;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub type BuiltinFn = Box<dyn Fn(Vec<Object>) -> Object + Sync + Send>;

lazy_static! {
    pub static ref BUILTIN_FUNCTIONS: Mutex<HashMap<String, BuiltinFn>> = {
        let mut m = HashMap::<String, BuiltinFn>::new();

        m.insert(
            "println".into(),
            Box::new(|args| {
                println!(
                    "{}",
                    args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                Object::Null
            }),
        );

        Mutex::new(m)
    };
}

pub struct Builtins;

impl Default for Builtins {
    fn default() -> Self {
        Self
    }
}

impl Builtins {
    pub fn has_fn(&self, name: &String) -> bool {
        let fns = BUILTIN_FUNCTIONS.lock().unwrap();
        fns.contains_key(name)
    }

    pub fn call(&self, name: String, args: Vec<Object>) -> Object {
        let fns = BUILTIN_FUNCTIONS.lock().unwrap();
        match fns.get(&name) {
            Some(f) => f(args),
            None => Object::Null,
        }
    }
}
