use std::any::Any;
use std::fmt::{Debug, Display};

pub mod ops;

pub trait BelalangType: Display + Debug {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn BelalangType {
    fn eq(&self, other: &Self) -> bool {
        // Ensure the types are the same
        self.type_name() == other.type_name()
            // Delegate to the underlying `PartialEq` implementation
            && format!("{}", self) == format!("{}", other)
    }
}
