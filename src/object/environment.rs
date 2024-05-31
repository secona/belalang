use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, super::Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Option<&super::Object> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: &String, value: super::Object) {
        self.store.insert(key.clone(), value);
    }
}
