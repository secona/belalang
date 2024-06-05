use std::io::{self, Write};

use super::object::Object;

pub struct Builtins {
    pub out: Box<dyn Write>,
}

impl Default for Builtins {
    fn default() -> Self {
        Self {
            out: Box::new(io::stdout()),
        }
    }
}

impl Builtins {
    pub fn new(out: Box<dyn Write>) -> Self {
        Self { out }
    }

    pub fn has_fn(&self, name: &String) -> bool {
        name == "println"
    }
    
    pub fn call(&mut self, name: String, args: Vec<Object>) -> Object {
        match name.as_str() {
            "println" => self.println(args),
            _ => Object::Null,
        }
    }

    pub fn println(&mut self, args: Vec<Object>) -> Object {
        let _ = write!(
            self.out,
            "{}\n",
            args.iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        Object::Null
    }
}
