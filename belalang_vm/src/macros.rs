#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! downcast {
    ($obj:ident, $type:ty) => {
        match $obj.as_any().downcast_ref::<$type>() {
            Some(obj) => obj,
            _ => return Err(RuntimeError::TypeError),
        }
    };
}

pub(crate) use downcast;
