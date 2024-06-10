use super::object::Object;
use std::collections::HashMap;

mod println;

pub trait BuiltinFunction {
    fn call(&self, args: Vec<Object>) -> Object;
}

pub struct Builtins {
    builtins: HashMap<String, Box<dyn BuiltinFunction>>,
}

impl Default for Builtins {
    fn default() -> Self {
        let mut builtins = HashMap::<String, Box<dyn BuiltinFunction>>::new();
        builtins.insert("println".into(), Box::new(println::Println));

        Self { builtins }
    }
}

impl Builtins {
    pub fn has_fn(&self, name: &String) -> bool {
        self.builtins.contains_key(name)
    }

    pub fn call(&mut self, name: String, args: Vec<Object>) -> Object {
        match self.builtins.get(&name) {
            Some(f) => f.call(args),
            None => Object::Null,
        }
    }

    pub fn override_builtin(&mut self, name: String, f: Box<dyn BuiltinFunction>) {
        if self.has_fn(&name) {
            self.builtins.insert(name, f);
        }
    }
}
