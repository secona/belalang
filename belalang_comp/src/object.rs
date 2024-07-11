#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub instructions: Vec<u8>,
    pub arity: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    Function(Function),
}
