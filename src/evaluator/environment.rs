use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Environment {
    stores: Vec<Rc<RefCell<HashMap<String, super::Object>>>>,
}

impl Default for Environment {
    fn default() -> Self {
        let stores = vec![Rc::new(RefCell::new(HashMap::new()))];
        Self { stores }
    }
}

impl Environment {
    pub fn capture(&self) -> Environment {
        let mut stores = Vec::with_capacity(self.stores.len());

        for store in &self.stores {
            stores.push(Rc::clone(store));
        }

        stores.push(Rc::new(RefCell::new(HashMap::new())));

        Environment { stores }
    }

    pub fn has(&self, key: &String) -> bool {
        for store in self.stores.iter().rev() {
            if store.borrow().contains_key(key) {
                return true;
            }
        }

        false
    }

    pub fn has_here(&self, key: &String) -> bool {
        if let Some(store) = self.stores.last() {
            return store.borrow().contains_key(key);
        }

        false
    }

    pub fn get<'a>(&'a self, key: &String) -> Option<Ref<'a, super::Object>> {
        for store in self.stores.iter().rev() {
            let value = Ref::filter_map(store.borrow(), |v| v.get(key)).ok();

            if let Some(v) = value {
                return Some(v);
            }
        }

        None
    }

    pub fn set(&mut self, key: &String, value: super::Object) {
        for store in self.stores.iter().rev() {
            let mut store = store.borrow_mut();
            if store.contains_key(key) {
                store.insert(key.clone(), value);
                return;
            }
        }

        if let Some(store) = self.stores.last_mut() {
            store.borrow_mut().insert(key.clone(), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    use crate::evaluator::Object;

    #[test]
    fn set() {
        let mut env = Environment::default();
        env.set(&String::from("name"), Object::Integer(10));

        assert_eq!(env.stores.len(), 1);
        let last_store = env.stores.last().unwrap().borrow();
        assert_eq!(*last_store.get("name").unwrap(), Object::Integer(10));
    }

    #[test]
    fn get() {
        let mut env = Environment::default();
        env.set(&"name".into(), Object::Integer(10));

        let value = env.get(&String::from("name")).unwrap();
        assert_eq!(*value, Object::Integer(10));
    }

    #[test]
    fn capture() {
        let mut env = Environment::default();
        env.set(&String::from("name"), Object::Integer(10));

        let captured_env = env.capture();
        env.set(&String::from("name"), Object::Integer(1));

        assert_eq!(
            *env.get(&"name".into()).unwrap(),
            *captured_env.get(&"name".into()).unwrap()
        );
    }
}
