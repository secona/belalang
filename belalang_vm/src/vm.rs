use belalang_devel::ops::*;

use crate::bytecode::Bytecode;
use crate::macros::downcast;
use crate::object::boolean::BelalangBoolean;
use crate::object::integer::BelalangInteger;
use crate::object::Object;
use crate::opcode;

use crate::error::RuntimeError;
use crate::mem::stack::{Stack, StackObject};

#[derive(Default)]
pub struct VM {
    pub ip: usize,
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,

    pub stack: Stack,
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

                opcode::ADD => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.add(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::SUB => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.sub(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::MUL => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.mul(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::DIV => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.div(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::MOD => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.r#mod(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::CONSTANT => {
                    let index = self.read_u16();
                    let constant = self.constants[index as usize].clone();

                    let object = match constant {
                        Object::Integer(int) => {
                            StackObject::Object(Box::new(BelalangInteger(int)))
                        }
                        Object::Boolean(boolean) => {
                            StackObject::Object(Box::new(BelalangBoolean(boolean)))
                        }
                        _ => panic!(),
                    };

                    self.stack.push(object)?;
                }

                opcode::TRUE => {
                    self.stack.push(StackObject::Object(Box::new(BelalangBoolean(true))))?;
                }

                opcode::FALSE => {
                    self.stack.push(StackObject::Object(Box::new(BelalangBoolean(false))))?;
                }

                opcode::NULL => {
                    self.stack.push(StackObject::Null)?;
                }

                opcode::EQUAL => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let result = BelalangBoolean(left == right);
                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::NOT_EQUAL => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let result = BelalangBoolean(left != right);
                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::LESS_THAN => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.lt(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::LESS_THAN_EQUAL => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.le(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::AND => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangBoolean);
                    let left = downcast!(left, BelalangBoolean);

                    let Ok(result) = left.and(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::OR => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangBoolean);
                    let left = downcast!(left, BelalangBoolean);

                    let Ok(result) = left.or(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::BIT_AND => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.bit_and(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::BIT_OR => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.bit_or(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::BIT_XOR => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.bit_xor(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::BIT_SL => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.bit_sl(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::BIT_SR => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };
                    let StackObject::Object(left) = left else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);
                    let left = downcast!(left, BelalangInteger);

                    let Ok(result) = left.bit_sr(right) else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::BANG => {
                    let right = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangBoolean);

                    let Ok(result) = right.not() else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::MINUS => {
                    let right = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };

                    let right = downcast!(right, BelalangInteger);

                    let Ok(result) = right.neg() else {
                        return Err(RuntimeError::TypeError);
                    };

                    self.stack.push(StackObject::Object(Box::new(result)))?;
                }

                opcode::JUMP => {
                    let relative = self.read_u16() as i16;
                    self.increment_ip(relative as usize);
                }

                opcode::JUMP_IF_FALSE => {
                    let relative = self.read_u16() as i16;

                    let right = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };

                    let value = downcast!(right, BelalangBoolean);

                    if !value.0 {
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
