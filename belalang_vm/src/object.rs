use std::fmt::Display;

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
    Builtin(usize),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => f.write_str("null"),
            Object::Integer(i) => f.write_str(&format!("{i}")),
            Object::Boolean(b) => f.write_str(&format!("{b}")),
            Object::Function(_) => f.write_str("<fn>"),
            Object::Builtin(_) => f.write_str("<builtin fn>"),
        }
    }
}
