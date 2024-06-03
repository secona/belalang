use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, super::Object>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn get(&self, key: &String) -> Option<&super::Object> {
        self.store.get(key)
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

    // #[test]
    // fn set_with_outer() {
    //     let mut outer = Environment::default();
    //     outer.set(&String::from("outer"), Object::Integer(1));
    //
    //     let mut env = Environment::subenv(&outer);
    //
    //     env.set(&String::from("name"), Object::Integer(10));
    //     let outer = env.outer.unwrap();
    //
    //     assert_eq!(env.store.len(), 1);
    //     assert_eq!(outer.store.len(), 1);
    //
    //     assert_eq!(*env.store.get("name").unwrap(), Object::Integer(10));
    //     assert_eq!(*outer.store.get("outer").unwrap(), Object::Integer(1));
    // }
    //
    // #[test]
    // fn get_with_outer() {
    //     let mut outer = Environment::default();
    //     outer.set(&String::from("outer"), Object::Integer(1));
    //
    //     let mut env = Environment::subenv(&outer);
    //
    //     env.set(&String::from("name"), Object::Integer(10));
    //     let env_value = env.get(&String::from("name")).unwrap();
    //     let outer_value = outer.get(&String::from("outer")).unwrap();
    //
    //     assert_eq!(*env_value, Object::Integer(10));
    //     assert_eq!(*outer_value, Object::Integer(1));
    // }
}
