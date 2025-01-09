#[derive(Debug, Default, Clone, PartialEq)]
pub enum Constant {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
}

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Constant>,
}
