use std::sync::{LazyLock, Mutex};
use std::collections::HashMap;

use super::object::BelalangObject;
use super::BelalangType;

pub type CastFn = fn(*const BelalangObject) -> Option<*const dyn BelalangType>;

pub static TYPE_REGISTRY: LazyLock<Mutex<HashMap<u32, CastFn>>> = 
    LazyLock::new(|| Mutex::new(HashMap::new()));

macro_rules! register_belalang_type {
    ($type:ty) => {
        const _: () = {
            #[ctor::ctor]
            fn register() { 
                $crate::TYPE_REGISTRY.lock().unwrap().insert(
                    <$type>::r#type(),
                    |obj: *const BelalangObject| -> Option<*const dyn BelalangType> {
                        Some(obj as *const $type)
                    }
                );
            }
        };
    };
}

pub(crate) use register_belalang_type;

/// # Safety
///
pub unsafe fn cast_type(obj: *const BelalangObject) -> Option<*const dyn BelalangType> {
    let obj_type = (*obj).obj_type;

    if let Some(cast_fn) = TYPE_REGISTRY.lock().unwrap().get(&obj_type) {
        cast_fn(obj)
    } else {
        None
    }
}
