use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};

const STACK_SIZE: usize = 4096;

pub struct StackAllocator {
    start: NonNull<u8>,
    top: NonNull<u8>,
    size: usize,
}

impl Default for StackAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl StackAllocator {
    pub fn new() -> Self {
        let layout = Layout::array::<u8>(STACK_SIZE).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };

        let ptr = match NonNull::new(ptr) {
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

    pub fn pop(&mut self) -> Option<u8> {
        if self.start == self.top {
            None
        } else {
            self.top = unsafe { NonNull::new_unchecked(self.top.as_ptr().sub(1)) };
            Some(unsafe { ptr::read(self.top.as_ptr()) })
        }
    }
}

impl Drop for StackAllocator {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.size).unwrap();
        unsafe { alloc::dealloc(self.start.as_ptr(), layout) }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::StackAllocator;

    #[test]
    fn allocator_push() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);

        let elem = unsafe { ptr::read(allocator.start.as_ptr()) };
        assert_eq!(elem, 10);
    }

    #[test]
    fn allocator_pop() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);
        allocator.push(11);
        allocator.push(12);

        assert_eq!(allocator.pop(), Some(12));
        assert_eq!(allocator.pop(), Some(11));
        assert_eq!(allocator.pop(), Some(10));
        assert_eq!(allocator.pop(), None);
    }

    #[test]
    fn allocator_drop() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);

        drop(allocator)
    }
}
