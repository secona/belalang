use std::any::Any;
use std::fmt::Display;
use std::ptr::NonNull;

use super::BelalangType;

#[repr(C)]
#[derive(Debug)]
pub struct BelalangObject {
    pub obj_type: u32,
    pub is_marked: bool,
    pub next: Option<NonNull<BelalangObject>>,
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

impl PartialEq for BelalangObject {
    fn eq(&self, other: &Self) -> bool {
        self.obj_type == other.obj_type
    }
}
