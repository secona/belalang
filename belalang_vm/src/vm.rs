use crate::builtins::BuiltinCollection;
use crate::bytecode::Bytecode;
use crate::object::Object;
use crate::opcode;

use crate::error::RuntimeError;
use crate::frame::FrameManager;

pub struct VM {
    pub constants: Vec<Object>,
    pub globals: Vec<Object>,

    pub last_popped: Object,
    pub stack: Vec<Object>,
    pub sp: usize,

    pub frame: FrameManager,

    pub builtin_collection: BuiltinCollection,
}

impl VM {
    pub fn run(&mut self, code: Bytecode) -> Result<(), RuntimeError> {
        self.constants.extend(code.constants.into_iter());
        self.frame
            .main_frame
            .function
            .instructions
            .extend(code.instructions.into_iter());

        while self.frame.current().ip < self.frame.current().ins().len() {
            let op = self.frame.current().ins()[self.frame.current().ip];

            match op {
                opcode::CONSTANT => {
                    let index = self.read_u16();
                    let object = self.constants[index as usize].clone();
                    self.push(object)?;
                }

                opcode::POP => {
                    self.last_popped = self.pop()?;
                }

                opcode::ADD => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_add(right)?)?;
                }

                opcode::SUB => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_sub(right)?)?;
                }

                opcode::MUL => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_mul(right)?)?;
                }

                opcode::DIV => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_div(right)?)?;
                }

                opcode::MOD => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_mod(right)?)?;
                }

                opcode::TRUE => {
                    self.push(Object::Boolean(true))?;
                }

                opcode::FALSE => {
                    self.push(Object::Boolean(false))?;
                }

                opcode::EQUAL => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(Object::Boolean(right == left))?;
                }

                opcode::NOT_EQUAL => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(Object::Boolean(right != left))?;
                }

                opcode::LESS_THAN => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_less_than(right)?)?;
                }

                opcode::LESS_THAN_EQUAL => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(left.try_less_than_equal(right)?)?;
                }

                opcode::AND => {
                    if let (Object::Boolean(right), Object::Boolean(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Boolean(right && left))?;
                    }
                }

                opcode::OR => {
                    if let (Object::Boolean(right), Object::Boolean(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Boolean(right || left))?;
                    }
                }

                opcode::BANG => {
                    if let Object::Boolean(b) = self.pop()? {
                        self.push(Object::Boolean(!b))?;
                    }
                }

                opcode::MINUS => {
                    if let Object::Integer(i) = self.pop()? {
                        self.push(Object::Integer(-i))?;
                    }
                }

                opcode::JUMP => {
                    let relative = self.read_u16() as i16;
                    self.inc_ip(relative as usize);
                }

                opcode::JUMP_IF_FALSE => {
                    let relative = self.read_u16() as i16;
                    let value = self.pop()?;

                    if let Object::Boolean(false) = value {
                        self.inc_ip(relative as usize);
                    }
                }

                opcode::NULL => {
                    self.push(Object::Null)?;
                }

                opcode::SET_GLOBAL => {
                    let index = self.read_u16() as usize;
                    let object = self.stack_top()?.clone();

                    match self.globals.get(index) {
                        Some(_) => self.globals[index] = object,
                        None => self.globals.insert(index, object),
                    }
                }

                opcode::GET_GLOBAL => {
                    let index = self.read_u16() as usize;
                    self.push(self.globals[index].clone())?;
                }

                opcode::SET_LOCAL => {
                    let index = self.read_u8() as usize;
                    let object = self.stack_top()?.clone();

                    let frame = self.frame.current_mut();
                    match frame.slots.get(index) {
                        Some(_) => frame.slots[index] = object,
                        None => frame.slots.insert(index, object),
                    }
                }

                opcode::GET_LOCAL => {
                    let index = self.read_u8() as usize;
                    self.push(self.frame.current().slots[index].clone())?;
                }

                opcode::GET_BUILTIN => {
                    let index = self.read_u8() as usize;
                    self.push(Object::Builtin(index))?;
                }

                opcode::CALL => {
                    match self.pop()? {
                        Object::Function(function) => {
                            let args: Vec<_> = (0..function.arity)
                                .map(|_| self.pop())
                                .collect::<Result<Vec<_>, _>>()?;

                            self.frame.push(function);

                            self.frame.current_mut().slots.extend(args);

                            continue; // continue because we dont want to increment the ip
                        }
                        Object::Builtin(index) => {
                            let args = (0..self.builtin_collection.get_arity(index)?)
                                .map(|_| self.pop())
                                .collect::<Result<Vec<_>, _>>()?;

                            let builtin = self.builtin_collection.get(index)?;
                            self.push(builtin.call(args))?;
                        }
                        _ => return Err(RuntimeError::NotAFunction),
                    }
                }

                opcode::RETURN_VALUE => {
                    if self.stack_top().is_err() {
                        self.push(Object::Null)?;
                    }

                    self.frame.pop();
                }

                _ => return Err(RuntimeError::UnknownInstruction(op)),
            };

            self.inc_ip(1)
        }

        Ok(())
    }

    pub fn inc_ip(&mut self, value: usize) {
        self.frame.current_mut().ip = self
            .frame
            .current()
            .ip
            .checked_add_signed(value as isize)
            .unwrap();
    }

    pub fn read_u16(&mut self) -> u16 {
        let current_frame = &mut self.frame.current_mut();

        let hi = current_frame.ins()[current_frame.ip + 1];
        let lo = current_frame.ins()[current_frame.ip + 2];
        current_frame.ip += 2;

        ((hi as u16) << 8) | (lo as u16)
    }

    pub fn read_u8(&mut self) -> u8 {
        let current_frame = &mut self.frame.current_mut();

        current_frame.ip += 1;
        current_frame.ins()[current_frame.ip]
    }

    pub fn stack_top(&mut self) -> Result<&Object, RuntimeError> {
        if self.sp == 0 {
            return Err(RuntimeError::StackUnderflow);
        }

        self.stack
            .get(self.sp - 1)
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn pop(&mut self) -> Result<Object, RuntimeError> {
        if self.sp == 0 {
            return Err(RuntimeError::StackUnderflow);
        }

        self.sp -= 1;

        Ok(self.stack.remove(self.sp))
    }

    fn push(&mut self, object: Object) -> Result<(), RuntimeError> {
        self.stack.push(object);
        self.sp += 1;

        Ok(())
    }
}

#[derive(Default)]
pub struct VMBuilder {
    builtin_collection: Option<BuiltinCollection>,
}

impl VMBuilder {
    pub fn builtin_collection(mut self, builtin_collection: BuiltinCollection) -> Self {
        self.builtin_collection = Some(builtin_collection);
        self
    }

    pub fn build(self) -> VM {
        let builtin_collection = self.builtin_collection.unwrap_or_default();
        let globals_offset = builtin_collection.keys().len(); // temporary fix

        VM {
            constants: Vec::new(),
            globals: vec![Object::Null; globals_offset],

            last_popped: Object::Null,
            stack: Vec::new(),
            sp: 0,

            frame: FrameManager::default(),

            builtin_collection,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{compiler::Compiler, object::Object};
//     use belalang_core::{lexer::Lexer, parser::Parser};
//
//     use super::VM;
//
//     #[test]
//     fn constant() {
//         let lexer = Lexer::new("5 + 10;".as_bytes());
//         let mut parser = Parser::new(lexer);
//         let program = parser.parse_program().unwrap();
//
//         let mut compiler = Compiler::default();
//         let mut code = compiler.compile_program(program).unwrap();
//
//         let mut vm = VM::default();
//         vm.append_code(&mut code);
//         vm.run().unwrap();
//
//         assert_eq!(vm.stack.len(), 0);
//
//         let Object::Integer(v) = vm.last_popped else {
//             panic!()
//         };
//         assert_eq!(v, 15);
//     }
// }
