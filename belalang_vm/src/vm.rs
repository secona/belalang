use std::ptr::NonNull;

use crate::bytecode::{Bytecode, Constant};
use crate::errors::RuntimeError;
use crate::mem::heap::Heap;
use crate::mem::stack::{Stack, StackObject};
use crate::opcode;
use crate::types::boolean::BelalangBoolean;
use crate::types::integer::BelalangInteger;
use crate::types::object::BelalangObject;
use crate::types::BelalangType;

#[derive(Default)]
pub struct VM {
    pub ip: usize,
    pub instructions: Vec<u8>,
    pub constants: Vec<Constant>,

    pub stack: Stack,
    pub heap: Heap,
}

impl VM {
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

                // NOTE: Next up, change the `Box::into_raw` into a call to
                // `self.heap.alloc`. I am using `Box::into_raw` because the
                // type implementation hasn't been updated.
                opcode::ADD => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.add(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::SUB => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.sub(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::MUL => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.mul(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::DIV => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.div(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::MOD => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.r#mod(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::CONSTANT => {
                    let index = self.read_u16();
                    let constant = self.constants[index as usize].clone();

                    let object = match constant {
                        Constant::Integer(int) => {
                            let ptr = self.heap.alloc(BelalangInteger::new(int))?;
                            StackObject::Object(ptr)
                        }
                        Constant::Boolean(boolean) => {
                            let ptr = self.heap.alloc(BelalangBoolean::new(boolean))?;
                            StackObject::Object(ptr)
                        }
                        _ => panic!(),
                    };

                    self.stack.push(object)?;
                }

                opcode::TRUE => {
                    let ptr = self.heap.alloc(BelalangBoolean::new(true))?;
                    self.stack.push(StackObject::Object(ptr))?;
                }

                opcode::FALSE => {
                    let ptr = self.heap.alloc(BelalangBoolean::new(false))?;
                    self.stack.push(StackObject::Object(ptr))?;
                }

                opcode::NULL => {
                    self.stack.push(StackObject::Null)?;
                }

                // NOTE: The EQUAL and NOT_EQUAL operator are currently
                // comparing types only. I still have no idea on how to
                // implement value comparison.
                opcode::EQUAL => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangObject;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangObject;
                        ptr.read()
                    };

                    let ptr = self.heap.alloc(BelalangBoolean::new(right == left))?;
                    self.stack.push(StackObject::Object(ptr))?;
                }

                opcode::NOT_EQUAL => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangObject;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangObject;
                        ptr.read()
                    };

                    let ptr = self.heap.alloc(BelalangBoolean::new(right != left))?;
                    self.stack.push(StackObject::Object(ptr))?;
                }

                opcode::LESS_THAN => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.lt(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::LESS_THAN_EQUAL => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.le(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::AND => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    let result = left.and(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::OR => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    let result = left.or(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::BIT_AND => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.bit_and(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::BIT_OR => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.bit_or(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::BIT_XOR => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.bit_xor(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::BIT_SL => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.bit_sl(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::BIT_SR => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let StackObject::Object(left) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let left = unsafe {
                        let ptr = left.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = left.bit_sr(&right)?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::BANG => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    let result = right.not()?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::MINUS => {
                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangInteger;
                        ptr.read()
                    };

                    let result = right.neg()?;
                    let ptr = Box::into_raw(result) as *mut BelalangObject;
                    self.stack
                        .push(StackObject::Object(NonNull::new(ptr).unwrap()))?;
                }

                opcode::JUMP => {
                    let relative = self.read_u16() as i16;
                    self.increment_ip(relative as usize);
                }

                opcode::JUMP_IF_FALSE => {
                    let relative = self.read_u16() as i16;

                    let StackObject::Object(right) = self.stack.pop()? else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = unsafe {
                        let ptr = right.as_ptr() as *const BelalangBoolean;
                        ptr.read()
                    };

                    if !right.truthy() {
                        self.increment_ip(relative as usize);
                    }
                }

                _ => return Err(RuntimeError::UnknownInstruction(op)),
            };

            self.increment_ip(1);
        }

        Ok(())
    }

    pub fn increment_ip(&mut self, value: usize) {
        self.ip = self.ip.checked_add_signed(value as isize).unwrap();
    }

    pub fn read_u16(&mut self) -> u16 {
        let hi = self.instructions[self.ip + 1];
        let lo = self.instructions[self.ip + 2];
        self.ip += 2;

        ((hi as u16) << 8) | (lo as u16)
    }

    pub fn read_u8(&mut self) -> u8 {
        let v = self.instructions[self.ip + 1];
        self.ip += 1;

        v
    }
}
