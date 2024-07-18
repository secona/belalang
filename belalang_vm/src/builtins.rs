use crate::{error::RuntimeError, object::Object};

pub type BuiltinFunction = Box<dyn Fn(Vec<Object>) -> Object>;

pub struct Builtin {
    pub name: String,
    pub arity: u8,
    pub function: BuiltinFunction,
}

impl Builtin {
    pub fn call(&self, args: Vec<Object>) -> Object {
        (self.function)(args)
    }
}

pub struct BuiltinCollection {
    pub store: Vec<Builtin>,
}

impl Default for BuiltinCollection {
    fn default() -> Self {
        let store = vec![Builtin {
            name: "print".into(),
            arity: 1,
            function: Box::new(|args| {
                println!("{}", args.first().unwrap());
                Object::Null
            }),
        }];

        Self { store }
    }
}

impl BuiltinCollection {
    pub fn get(&self, index: usize) -> Result<&Builtin, RuntimeError> {
        self.store.get(index).ok_or(RuntimeError::UnknownBuiltinFunction)
    }

    pub fn get_arity(&self, index: usize) -> Result<usize, RuntimeError> {
        let builtin = self.get(index)?;
        Ok(builtin.arity as usize)
    }
}
