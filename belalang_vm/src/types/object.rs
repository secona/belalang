use std::any::Any;
use std::fmt::Display;
use std::ptr::NonNull;

use belalang_macros::register_belalang_type;

use super::BelalangType;

#[register_belalang_type]
pub struct BelalangObject {
    pub obj_type: u32,
    pub is_marked: bool,
    pub next: Option<NonNull<BelalangObject>>,
}

impl BelalangObject {
    pub fn new<T: BelalangType>() -> Self {
        Self {
            obj_type: T::r#type(),
            is_marked: false,
            next: None,
        }
    }
}

impl Display for BelalangObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object")
    }
}

impl BelalangType for BelalangObject {
    fn type_name() -> String
    where
        Self: Sized,
    {
        "Object".into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
