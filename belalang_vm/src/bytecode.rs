use crate::object::Object;

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}
