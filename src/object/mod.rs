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

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
