pub mod boolean;
pub mod integer;

use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Object {
    #[default]
    Null,
    Integer(i64),
    Boolean(bool),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(i) => write!(f, "{i}"),
            Object::Boolean(b) => write!(f, "{b}"),
        }
    }
}
