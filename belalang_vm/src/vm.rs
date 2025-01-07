use crate::bytecode::Bytecode;
use crate::globals::Globals;
use crate::object::{Object, ObjTrait};
use crate::opcode;

use crate::error::RuntimeError;
use crate::stack::Stack;

pub struct VM {
    pub ip: usize,
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,

    pub stack: Stack,
    pub globals: Globals,
}

impl Default for VM {
    fn default() -> Self {
        let globals_offset = crate::builtins::BUILTIN_FUNCTIONS.len(); // temporary fix

        VM {
            ip: 0,
            instructions: Vec::new(),
            constants: Vec::new(),

            stack: Stack::default(),
            globals: Globals::with_offset(globals_offset),
        }
    }
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
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_add(right)?)?;
                }

                opcode::SUB => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_sub(right)?)?;
                }

                opcode::MUL => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_mul(right)?)?;
                }

                opcode::DIV => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_div(right)?)?;
                }

                opcode::MOD => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_mod(right)?)?;
                }

                opcode::CONSTANT => {
                    let index = self.read_u16();
                    let object = self.constants[index as usize].clone();
                    self.stack.push(object)?;
                }

                opcode::TRUE => {
                    self.stack.push(Object::Boolean(true))?;
                }

                opcode::FALSE => {
                    self.stack.push(Object::Boolean(false))?;
                }

                opcode::NULL => {
                    self.stack.push(Object::Null)?;
                }

                opcode::EQUAL => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(Object::Boolean(right == left))?;
                }

                opcode::NOT_EQUAL => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(Object::Boolean(right != left))?;
                }

                opcode::LESS_THAN => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_less_than(right)?)?;
                }

                opcode::LESS_THAN_EQUAL => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_less_than_equal(right)?)?;
                }

                opcode::AND => {
                    if let (Object::Boolean(right), Object::Boolean(left)) =
                        (self.stack.pop_take()?, self.stack.pop_take()?)
                    {
                        self.stack.push(Object::Boolean(right && left))?;
                    }
                }

                opcode::OR => {
                    if let (Object::Boolean(right), Object::Boolean(left)) =
                        (self.stack.pop_take()?, self.stack.pop_take()?)
                    {
                        self.stack.push(Object::Boolean(right || left))?;
                    }
                }

                opcode::BIT_AND => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_bit_and(right)?)?;
                }

                opcode::BIT_OR => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_bit_or(right)?)?;
                }

                opcode::BIT_XOR => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_bit_xor(right)?)?;
                }

                opcode::BIT_SL => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_bit_sl(right)?)?;
                }

                opcode::BIT_SR => {
                    let right = self.stack.pop_take()?;
                    let left = self.stack.pop_take()?;
                    self.stack.push(left.try_bit_sr(right)?)?;
                }

                opcode::BANG => {
                    if let Object::Boolean(b) = self.stack.pop_take()? {
                        self.stack.push(Object::Boolean(!b))?;
                    }
                }

                opcode::MINUS => {
                    if let Object::Integer(i) = self.stack.pop_take()? {
                        self.stack.push(Object::Integer(-i))?;
                    }
                }

                opcode::JUMP => {
                    let relative = self.read_u16() as i16;
                    self.increment_ip(relative as usize);
                }

                opcode::JUMP_IF_FALSE => {
                    let relative = self.read_u16() as i16;
                    let value = self.stack.pop_take()?;

                    if let Object::Boolean(false) = value {
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
