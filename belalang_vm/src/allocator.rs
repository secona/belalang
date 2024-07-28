use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};

const STACK_SIZE: usize = 4096;

pub struct StackAllocator {
    start: NonNull<u8>,
    top: NonNull<u8>,
    size: usize,
}

impl StackAllocator {
    pub fn new() -> Self {
        let layout = Layout::array::<u8>(STACK_SIZE).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };

        let ptr = match NonNull::new(ptr as *mut u8) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };

        Self {
            start: ptr,
            top: ptr,
            size: STACK_SIZE,
        }
    }

    pub fn push(&mut self, elem: u8) {
        let top = self.top.as_ptr();
        let start = self.start.as_ptr();

        if unsafe { top.offset_from(start) } > self.size as isize {
            panic!("stack overflow");
        }

        self.top = unsafe { NonNull::new_unchecked(top.add(1)) };
        unsafe { ptr::write(top, elem) }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::StackAllocator;

    #[test]
    fn allocator() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);

        let elem = unsafe { ptr::read(allocator.start.as_ptr()) };
        assert_eq!(elem, 10);
    }
}
