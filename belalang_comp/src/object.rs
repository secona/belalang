#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    pub instructions: Vec<u8>,
    pub arity: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Object {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
    Function(Function),
}
