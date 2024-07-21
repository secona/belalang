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
    Float(f64),
    Boolean(bool),
    String(String),
    Function(Function),
    Builtin(usize),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => f.write_str("null"),
            Object::Integer(i) => f.write_str(&format!("{i}")),
            Object::Float(fl) => f.write_str(&format!("{fl}")),
            Object::Boolean(b) => f.write_str(&format!("{b}")),
            Object::String(s) => f.write_str(&s),
            Object::Function(_) => f.write_str("<fn>"),
            Object::Builtin(_) => f.write_str("<builtin fn>"),
        }
    }
}
