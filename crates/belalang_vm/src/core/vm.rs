use crate::core::bytecode::{Bytecode, Constant};
use crate::core::opcode;
use crate::errors::RuntimeError;
use crate::mem::heap::Heap;
use crate::mem::stack::{Stack, StackValue};

use crate::objects::array::BelalangArray;
use crate::objects::boolean::BelalangBoolean;
use crate::objects::integer::BelalangInteger;

macro_rules! pop_object {
    ($self:expr) => {
        if let Ok(StackValue::ObjectPtr(obj)) = $self.stack.pop() {
            obj.as_ptr()
        } else {
            return Err(RuntimeError::TypeError);
        }
    };
}

/// The core Virtual Machine structure.
///
/// # Note
/// The `stack` and `heap` are public to facilitate integration tests in
/// `crates/belalang_vm/tests`. However, changes need to be made to make
/// these fields private.
#[derive(Default)]
pub struct VM {
    /// The current instruction the VM is pointing to. This is why it's
    /// named `ip` (short for instruction pointer).
    ip: usize,

    /// The list of bytecode instructions the VM is executing. The value of
    /// this field is supplied through the [`Bytecode`] struct.
    instructions: Vec<u8>,

    /// The list of constants the VM is working with. The value of this field
    /// is supplied through the [`Bytecode`] struct.
    constants: Vec<Constant>,

    /// The stack memory of the VM.
    stack: Stack,

    /// The heap memory of the VM. Made `pub(crate)` to support allocations in [crate::objects].
    pub(crate) heap: Heap,
}

impl VM {
    /// Executes the provided [`Bytecode`] program.
    ///
    /// # Arguments
    /// * `code` -- The [`Bytecode`] to be executed.
    ///
    /// # Example
    /// ```rust,ignore
    /// let constants = vec![Constant::Integer(12), Constant::Integer(5)];
    ///
    /// let mut instructions = Vec::new();
    /// instructions.extend(opcode::constant(0));
    /// instructions.extend(opcode::constant(1));
    /// instructions.push(opcode::POP);
    ///
    /// let mut vm = VM::default();
    ///
    /// vm.run(Bytecode {
    ///     instructions,
    ///     constants,
    /// })
    ///
    /// ```
    ///
    /// # Note
    /// In the example above, the [`Bytecode`] is generated manually. This is
    /// not representative of real world situations. Instead the [`Bytecode`]
    /// should be generated using tools, such as `belalang_compiler`.
    pub fn run(&mut self, code: Bytecode) -> Result<(), RuntimeError> {
        self.constants.extend(code.constants);
        self.instructions.extend(code.instructions);

        while self.ip < self.instructions.len() {
            let op = self.instructions[self.ip];

            match op {
                opcode::NOOP => {}

                opcode::POP => {
                    self.stack.pop()?;
                }

                opcode::ADD => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).add(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::SUB => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).sub(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::MUL => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).mul(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::DIV => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).div(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::MOD => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).r#mod(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::CONSTANT => {
                    let index = self.read_u16();
                    let constant = self.constants[index as usize].clone();

                    let object = match constant {
                        Constant::Integer(int) => {
                            let ptr = self.heap.alloc(BelalangInteger::new(int))?;
                            StackValue::ObjectPtr(ptr)
                        }
                        Constant::Boolean(boolean) => {
                            let ptr = self.heap.alloc(BelalangBoolean::new(boolean))?;
                            StackValue::ObjectPtr(ptr)
                        }
                        Constant::String(_) => todo!(),
                        Constant::Null => todo!(),
                    };

                    self.stack.push(object)?;
                }

                opcode::TRUE => {
                    let ptr = self.heap.alloc(BelalangBoolean::new(true))?;
                    self.stack.push(StackValue::ObjectPtr(ptr))?;
                }

                opcode::FALSE => {
                    let ptr = self.heap.alloc(BelalangBoolean::new(false))?;
                    self.stack.push(StackValue::ObjectPtr(ptr))?;
                }

                opcode::NULL => {
                    self.stack.push(StackValue::Null)?;
                }

                opcode::EQUAL => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).eq(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::NOT_EQUAL => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).ne(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::LESS_THAN => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).lt(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::LESS_THAN_EQUAL => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).le(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::AND => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).and(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::OR => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).or(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::BIT_AND => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).bit_and(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::BIT_OR => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).bit_or(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::BIT_XOR => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).bit_xor(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::BIT_SL => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).bit_sl(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::BIT_SR => {
                    let right = pop_object!(self);
                    let left = pop_object!(self);

                    let result = unsafe { (*left).bit_sr(self, &*right) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::BANG => {
                    let right = pop_object!(self);

                    let result = unsafe { (*right).not(self) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::MINUS => {
                    let right = pop_object!(self);

                    let result = unsafe { (*right).neg(self) }?;

                    self.stack.push(StackValue::ObjectPtr(result))?;
                }

                opcode::MAKE_ARRAY => {
                    let cap: usize = self.read_u8().into();
                    let array = self.heap.alloc(BelalangArray::with_capacity(cap))?;

                    for i in 0..cap {
                        let Ok(StackValue::ObjectPtr(obj)) = self.stack.pop() else {
                            return Err(RuntimeError::TypeError);
                        };

                        unsafe {
                            (*(array.as_ptr() as *mut BelalangArray))
                                .ptr
                                .add(i)
                                .write(obj)
                        };
                    }

                    self.stack.push(StackValue::ObjectPtr(array))?;
                }

                opcode::JUMP => {
                    let relative = self.read_u16() as i16;
                    self.increment_ip(relative as usize);
                }

                opcode::JUMP_IF_FALSE => {
                    let relative = self.read_u16() as i16;

                    let right = pop_object!(self);

                    if unsafe { !(*right).truthy() } {
                        self.increment_ip(relative as usize);
                    }
                }

                _ => return Err(RuntimeError::UnknownInstruction(op)),
            };

            self.increment_ip(1);
        }

        Ok(())
    }

    fn increment_ip(&mut self, value: usize) {
        self.ip = self.ip.checked_add_signed(value as isize).unwrap();
    }

    fn read_u16(&mut self) -> u16 {
        let hi = self.instructions[self.ip + 1];
        let lo = self.instructions[self.ip + 2];
        self.ip += 2;

        ((hi as u16) << 8) | (lo as u16)
    }

    fn read_u8(&mut self) -> u8 {
        let v = self.instructions[self.ip + 1];
        self.ip += 1;

        v
    }

    pub fn stack_size(&self) -> usize {
        self.stack.size()
    }

    pub fn stack_pop(&mut self) -> Result<StackValue, RuntimeError> {
        self.stack.pop()
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        self.instructions.clear();
        self.constants.clear();

        std::mem::drop(std::mem::take(&mut self.stack));
        std::mem::drop(std::mem::take(&mut self.heap));
    }
}
