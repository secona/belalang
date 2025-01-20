use crate::bytecode::{Bytecode, Constant};
use crate::errors::RuntimeError;
use crate::mem::stack::{Stack, StackObject};
use crate::opcode;
use crate::types::boolean::BelalangBoolean;
use crate::types::integer::BelalangInteger;

#[derive(Default)]
pub struct VM {
    pub ip: usize,
    pub instructions: Vec<u8>,
    pub constants: Vec<Constant>,

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

                    let result = left.add(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.sub(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.mul(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.div(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.r#mod(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
                }

                opcode::CONSTANT => {
                    let index = self.read_u16();
                    let constant = self.constants[index as usize].clone();

                    let object = match constant {
                        Constant::Integer(int) => {
                            StackObject::Object(Box::new(BelalangInteger::new(int)))
                        }
                        Constant::Boolean(boolean) => {
                            StackObject::Object(Box::new(BelalangBoolean::new(boolean)))
                        }
                        _ => panic!(),
                    };

                    self.stack.push(object)?;
                }

                opcode::TRUE => {
                    self.stack
                        .push(StackObject::Object(Box::new(BelalangBoolean::new(true))))?;
                }

                opcode::FALSE => {
                    self.stack
                        .push(StackObject::Object(Box::new(BelalangBoolean::new(false))))?;
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

                    let result = BelalangBoolean::new(left == right);
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

                    let result = BelalangBoolean::new(left != right);
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

                    let result = left.lt(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.le(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.and(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.or(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.bit_and(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.bit_or(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.bit_xor(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.bit_sl(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
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

                    let result = left.bit_sr(&*right)?;
                    self.stack.push(StackObject::Object(result))?;
                }

                opcode::BANG => {
                    let right = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };

                    let result = right.not()?;
                    self.stack.push(StackObject::Object(result))?;
                }

                opcode::MINUS => {
                    let right = self.stack.pop()?;

                    let StackObject::Object(right) = right else {
                        return Err(RuntimeError::TypeError);
                    };

                    let result = right.neg()?;
                    self.stack.push(StackObject::Object(result))?;
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
