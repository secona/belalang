#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! downcast {
    ($obj:ident, $type:ty) => {
        match $obj.as_any().downcast_ref::<$type>() {
            Some(obj) => obj,
            // TODO: Change this to the correct runtime error
            _ => return Err(RuntimeError::IntegerOverflow),
        }
    };
}

pub(crate) use downcast;

