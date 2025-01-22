use std::sync::{LazyLock, Mutex};
use std::collections::HashMap;

use super::object::BelalangObject;
use super::BelalangType;

pub type CastFn = fn(*const BelalangObject) -> Option<*const dyn BelalangType>;

pub static TYPE_REGISTRY: LazyLock<Mutex<HashMap<u32, CastFn>>> = 
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register_type<T: BelalangType + 'static>() {
    TYPE_REGISTRY.lock().unwrap().insert(T::r#type(), |obj: *const BelalangObject| {
        Some(obj as *const T)
    });
}

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
