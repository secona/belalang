use std::any::Any;
use std::fmt::{Debug, Display};

pub mod ops;
pub mod errors;

pub trait BelalangType: Display + Debug {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;

    fn equal_type(&self, other: &dyn BelalangType) -> bool {
        self.type_name() == other.type_name()
    }
}

impl PartialEq for dyn BelalangType {
    fn eq(&self, other: &Self) -> bool {
        // Temporary implementation
        self.equal_type(other) && format!("{}", self) == format!("{}", other)
    }
}
