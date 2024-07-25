use crate::object::Object;

#[derive(Debug)]
pub struct Globals {
    globals: Vec<Object>,
}

impl Globals {
    pub fn with_offset(size: usize) -> Self {
        Self {
            globals: vec![Object::Null; size],
        }
    }

    pub fn set(&mut self, index: usize, object: Object) {
        match self.globals.get(index) {
            Some(_) => self.globals[index] = object,
            None => self.globals.insert(index, object),
        }
    }

    pub fn get(&self, index: usize) -> Object {
        self.globals[index].clone()
    }
}
