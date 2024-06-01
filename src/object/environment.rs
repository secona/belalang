use std::collections::HashMap;

pub struct Environment<'a> {
    store: HashMap<String, super::Object>,
    outer: Option<&'a Environment<'a>>,
}

impl Default for Environment<'_> {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }
}

impl<'a> Environment<'a> {
    pub fn subenv(outer: &'a Environment<'a>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, key: &String) -> Option<&super::Object> {
        match self.store.get(key) {
            Some(value) => Some(value),
            None => match self.outer {
                Some(outer) => outer.get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: &String, value: super::Object) {
        self.store.insert(key.clone(), value);
    }
}

#[cfg(test)]
mod tests {
    use crate::object::Object;

    use super::Environment;

    #[test]
    fn set() {
        let mut env = Environment::default();

        env.set(&String::from("name"), Object::Integer(10));

        assert_eq!(env.store.len(), 1);
        assert_eq!(*env.store.get("name").unwrap(), Object::Integer(10));
    }

    #[test]
    fn get() {
        let mut env = Environment::default();

        env.set(&String::from("name"), Object::Integer(10));

        let value = env.get(&String::from("name")).unwrap();

        assert_eq!(*value, Object::Integer(10));
    }

    #[test]
    fn set_with_outer() {
        let mut outer = Environment::default();
        outer.set(&String::from("outer"), Object::Integer(1));

        let mut env = Environment::subenv(&outer);

        env.set(&String::from("name"), Object::Integer(10));
        let outer = env.outer.unwrap();

        assert_eq!(env.store.len(), 1);
        assert_eq!(outer.store.len(), 1);

        assert_eq!(*env.store.get("name").unwrap(), Object::Integer(10));
        assert_eq!(*outer.store.get("outer").unwrap(), Object::Integer(1));
    }

    #[test]
    fn get_with_outer() {
        let mut outer = Environment::default();
        outer.set(&String::from("outer"), Object::Integer(1));

        let mut env = Environment::subenv(&outer);

        env.set(&String::from("name"), Object::Integer(10));
        let env_value = env.get(&String::from("name")).unwrap();
        let outer_value = outer.get(&String::from("outer")).unwrap();

        assert_eq!(*env_value, Object::Integer(10));
        assert_eq!(*outer_value, Object::Integer(1));
    }
}
