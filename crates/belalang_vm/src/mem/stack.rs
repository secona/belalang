use crate::errors::RuntimeError;
use crate::core::BelalangPtr;

/// Default stack size of Belalang VM
///
/// This is currently a good enough stack size for Belalang VM. I don't really know how stack sizes
/// are supposed to be implemented for efficiency. I still have stuff to read and things to
/// explore.
const STACK_SIZE: usize = 4096;

/// Objects that live on the stack
///
/// # Problems
/// - Should probably rename this to not have "Object", maybe "StackValue" would be good.
/// - Namings of each variant should be more descriptive.
#[derive(Default, Debug)]
pub enum StackObject {
    /// Pointer to an object, like [`BelalangInteger`][crate::objects::integer::BelalangInteger]
    Object(BelalangPtr),

    /// Pointer to an address in the bytecode
    Ptr(u8),
    
    /// Null value in the stack
    ///
    /// This value is mostly used to indicate uninitialized variables and actual null values.
    ///
    /// # Problems
    /// - I am not sure to go forward with the "null" name or not.
    /// - I am not sure if uninitialized variables should have this as their value.
    #[default]
    Null,
}

const DEFAULT_STACK_VALUE: StackObject = StackObject::Null;

/// Belalang VM's stack implementation
///
/// This stack is both the call stack and the frame stack.
pub struct Stack {
    stack: [StackObject; STACK_SIZE],
    cap: usize,
    sp: usize,
    fp: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        while self.pop().is_ok() {}
    }
}

impl Stack {
    /// Creates a new stack
    ///
    /// Pretty self explainatory.
    pub fn new() -> Self {
        Self {
            stack: [DEFAULT_STACK_VALUE; STACK_SIZE],
            cap: STACK_SIZE,
            sp: 0,
            fp: 0,
        }
    }

    /// Returns the stack size
    pub fn size(&self) -> usize {
        self.sp
    }

    /// Pushes a new [`StackObject`] to the stack
    pub fn push(&mut self, elem: StackObject) -> Result<(), RuntimeError> {
        if self.sp >= self.cap {
            return Err(RuntimeError::StackOverflow);
        }

        self.stack[self.sp] = elem;
        self.sp += 1;

        Ok(())
    }

    /// Pops a [`StackObject`] from the stack
    ///
    /// This function uses [`std::mem::take`] to get the top-most value of the stack, leaving a
    /// [`StackObject::Null`] behind.
    pub fn pop(&mut self) -> Result<StackObject, RuntimeError> {
        if self.sp == 0 {
            Err(RuntimeError::StackUnderflow)
        } else {
            self.sp -= 1;
            Ok(std::mem::take(&mut self.stack[self.sp]))
        }
    }

    /// Gets the top-most [`StackObject`] value from the stack
    ///
    /// Returns the reference to the top-most value, and does not remove it.
    pub fn top(&mut self) -> Option<&StackObject> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }

    /// Pushes a new stack frame to the stack
    ///
    /// Typically used when going into a function scope.
    pub fn push_frame(&mut self, locals_count: u8, return_address: u8) -> Result<(), RuntimeError> {
        self.push(StackObject::Ptr(return_address))?;
        self.push(StackObject::Ptr(self.fp as u8))?;
        self.fp = self.sp;

        for _ in 0..locals_count {
            self.push(StackObject::Null)?;
        }

        Ok(())
    }

    /// Pops a stack frame from the stack
    ///
    /// Typically used when going out of a function scope
    pub fn pop_frame(&mut self) -> Result<StackObject, RuntimeError> {
        self.sp = self.fp;

        if let StackObject::Ptr(v) = self.pop()? {
            self.fp = v.into();
        }

        self.pop()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_allocation)]

    use crate::mem::heap::Heap;
    use crate::objects::integer::BelalangInteger;

    use super::*;

    macro_rules! assert_belalang_integer {
        ($top:expr, $value:expr) => {
            let StackObject::Object(obj) = $top else {
                panic!("Not a StackObject::Object");
            };

            let int = unsafe { (obj.as_ptr() as *const BelalangInteger).read() };
            assert_eq!(int.value, $value);
        };
    }

    #[test]
    fn push() {
        let mut stack = Stack::new();
        let mut heap = Heap::default();

        let ptr = heap.alloc(BelalangInteger::new(10)).unwrap();
        stack.push(StackObject::Object(ptr)).unwrap();

        assert_belalang_integer!(&stack.top().unwrap(), 10);

        drop(stack);
        drop(heap);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        let mut heap = Heap::default();

        let ptr = heap.alloc(BelalangInteger::new(10)).unwrap();
        stack.push(StackObject::Object(ptr)).unwrap();

        let ptr = heap.alloc(BelalangInteger::new(11)).unwrap();
        stack.push(StackObject::Object(ptr)).unwrap();

        let ptr = heap.alloc(BelalangInteger::new(12)).unwrap();
        stack.push(StackObject::Object(ptr)).unwrap();

        assert_belalang_integer!(&stack.pop().unwrap(), 12);
        assert_belalang_integer!(&stack.pop().unwrap(), 11);
        assert_belalang_integer!(&stack.pop().unwrap(), 10);

        assert!(matches!(stack.pop(), Err(RuntimeError::StackUnderflow)));
    }

    #[test]
    fn push_frame() {
        let mut stack = Stack::new();

        stack.push_frame(3, 12).unwrap();

        assert_eq!(stack.fp, 2);
        assert_eq!(stack.sp, 5);

        assert!(matches!(stack.pop().unwrap(), StackObject::Null)); // local 1
        assert!(matches!(stack.pop().unwrap(), StackObject::Null)); // local 2
        assert!(matches!(stack.pop().unwrap(), StackObject::Null)); // local 3
        assert!(matches!(stack.pop().unwrap(), StackObject::Ptr(0))); // fp
        assert!(matches!(stack.pop().unwrap(), StackObject::Ptr(12))); // return address
        assert!(matches!(stack.pop(), Err(RuntimeError::StackUnderflow))); // bottom of stack
    }

    #[test]
    fn pop_frame() {
        let mut stack = Stack::new();

        stack.push_frame(3, 12).unwrap();
        stack.pop_frame().unwrap();

        assert_eq!(stack.sp, 0);
        assert_eq!(stack.fp, 0);
    }
}
