use std::cell::RefCell;

pub use errors::*;
pub use heap::*;
pub use ptr::*;

mod errors;
mod heap;
mod ptr;

thread_local! {
    static HEAP: RefCell<GcHeap> = RefCell::new(GcHeap::default());
}

pub fn with_heap<F, R>(f: F) -> R
where
    F: FnOnce(&mut GcHeap) -> R,
{
    HEAP.with(|h| f(&mut h.borrow_mut()))
}
