use std::fmt::Display;

use belalang_macros::belalang_type;

use crate::types::object::BelalangObject;
use crate::types::BelalangType;

// NOTE: I don't know whether or not it is better to implement my own strings
// from scratch with pointers and len and etc, or to just use Rust's builtin
// strings.

#[belalang_type]
pub struct BelalangString {
    pub string: String,
}

impl BelalangString {
    pub fn new(string: String) -> Self {
        Self {
            base: BelalangObject {
                obj_type: Self::r#type(),
                is_marked: false,
                next: None,
            },
            string,
        }
    }
}

impl Display for BelalangString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl BelalangType for BelalangString {
    fn type_name() -> String
    where
        Self: Sized
    {
        "String".into()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
