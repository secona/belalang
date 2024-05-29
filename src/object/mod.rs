mod boolean;
mod integer;
mod null;

pub use boolean::*;
pub use integer::*;
pub use null::*;

pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

pub trait ObjectTrait {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(int) => f.write_str(&format!("Object::Integer({})", int.value)),
            Self::Boolean(bool) => f.write_str(&format!("Object::Boolean({})", bool.value)),
            Self::Null(_) => f.write_str(&format!("Object::Null")),
        }
    }
}
