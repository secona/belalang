#[derive(Debug, Default, Clone, PartialEq)]
pub enum Constant {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
    String(&'static str),
}

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Constant>,
}
