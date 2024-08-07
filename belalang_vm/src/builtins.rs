use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::object::Object;

use indexmap::{indexmap, IndexMap};

pub type BuiltinFunction = Box<dyn Fn(Vec<Object>) -> Object>;

pub struct Builtin {
    pub arity: u8,
    pub function: BuiltinFunction,
}

impl Builtin {
    pub fn call(&self, args: Vec<Object>) -> Object {
        (self.function)(args)
    }
}

pub struct BuiltinCollection {
    store: IndexMap<String, Builtin>,
}

impl Default for BuiltinCollection {
    fn default() -> Self {
        Self {
            store: indexmap! {
                "print".into() => Builtin {
                    arity: 1,
                    function: Box::new(|args| {
                        println!("{}", args.first().unwrap());
                        Object::Null
                    })
                }
            },
        }
    }
}

impl BuiltinCollection {
    pub fn keys(&self) -> Vec<&String> {
        self.store.iter().map(|kv| kv.0).collect()
    }

    pub fn get(&self, index: usize) -> Result<&Builtin, RuntimeError> {
        self.store
            .get_index(index)
            .map(|kv| kv.1)
            .ok_or(RuntimeError::UnknownBuiltinFunction)
    }

    pub fn get_arity(&self, index: usize) -> Result<usize, RuntimeError> {
        let builtin = self.get(index)?;
        Ok(builtin.arity as usize)
    }
}

#[derive(Default)]
pub struct BuiltinCollectionBuilder {
    store: HashMap<String, Builtin>,
}

impl BuiltinCollectionBuilder {
    pub fn builtin_function(mut self, key: String, value: Builtin) -> Self {
        self.store.insert(key, value);
        self
    }

    pub fn build(self) -> BuiltinCollection {
        let mut bc = BuiltinCollection::default();

        for (k, v) in self.store.into_iter() {
            bc.store.entry(k).and_modify(|b| *b = v);
        }

        bc
    }
}
