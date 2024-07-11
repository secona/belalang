use belalang_comp::code;
use belalang_comp::compiler::Compiler;
use belalang_comp::object::Object;

use crate::error::RuntimeError;
use crate::frame::FrameManager;

pub struct VM {
    pub constants: Vec<Object>,
    pub globals: Vec<Object>,

    pub last_popped: Object,
    pub stack: Vec<Object>,
    pub sp: usize,

    pub frame: FrameManager,
}

impl VM {
    pub fn new(mut compiler: Compiler) -> Self {
        Self {
            constants: compiler.constants.drain(..).collect(),
            globals: vec![Object::Null; compiler.scope.current().symbol_count],

            last_popped: Object::Null,
            stack: Vec::new(),
            sp: 0,

            frame: FrameManager::new(compiler),
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        let mut ip = 0;
        while ip < self.frame.current().ins().len() {
            let op = self.frame.current().ins()[ip];

            match op {
                code::CONSTANT => {
                    let index = self.read_u16(&mut ip);
                    let object = self.constants[index as usize].clone();
                    self.push(object)?;
                }

                code::POP => {
                    self.last_popped = self.pop()?;
                }

                code::ADD => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left + right))?;
                    };
                }

                code::SUB => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left - right))?;
                    };
                }

                code::MUL => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left * right))?;
                    };
                }

                code::DIV => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left / right))?;
                    };
                }

                code::MOD => {
                    if let (Object::Integer(right), Object::Integer(left)) =
                        (self.pop()?, self.pop()?)
                    {
                        self.push(Object::Integer(left % right))?;
                    };
                }

                code::TRUE => {
                    self.push(Object::Boolean(true))?;
                }

                code::FALSE => {
                    self.push(Object::Boolean(false))?;
                }

                code::EQ => todo!(),

                code::NE => todo!(),

                code::LT => todo!(),

                code::LE => todo!(),

                code::GT => todo!(),

                code::GE => todo!(),

                code::BANG => todo!(),

                code::MINUS => todo!(),

                code::JUMP => {
                    let dest = self.read_u16(&mut ip);
                    ip = (dest - 1) as usize;
                }

                code::JUMP_IF_FALSE => {
                    let dest = self.read_u16(&mut ip);
                    let value = self.pop()?;

                    if let Object::Boolean(false) = value {
                        ip = (dest - 1) as usize;
                    }
                }

                code::NULL => todo!(),

                // temporary
                code::DEF_GLOBAL | code::SET_GLOBAL => {
                    let index = self.read_u16(&mut ip) as usize;
                    let object = self.stack_top()?.clone();
                    self.globals[index] = object;
                }

                code::GET_GLOBAL => {
                    let index = self.read_u16(&mut ip) as usize;
                    self.push(self.globals[index].clone())?;
                }

                code::SET_LOCAL => {
                    let index = self.read_u8(&mut ip) as usize;
                    let object = self.stack_top()?.clone();
                    self.frame.current_mut().slots.insert(index, object);
                }

                code::GET_LOCAL => {
                    let index = self.read_u8(&mut ip) as usize;
                    self.push(self.frame.current().slots[index].clone())?;
                }

                code::CALL => todo!(),

                code::RETURN => todo!(),

                code::RETURN_VALUE => todo!(),

                _ => return Err(RuntimeError::UnknownInstruction(op)),
            };

            ip += 1;
        }

        Ok(())
    }

    pub fn read_u16(&mut self, ip: &mut usize) -> u16 {
        let hi = self.frame.current().ins()[*ip + 1];
        let lo = self.frame.current().ins()[*ip + 2];
        *ip += 2;

        ((hi as u16) << 8) | (lo as u16)
    }

    pub fn read_u8(&mut self, ip: &mut usize) -> u8 {
        *ip += 1;
        self.frame.current().ins()[*ip]
    }

    pub fn stack_top(&mut self) -> Result<&Object, RuntimeError> {
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

#[cfg(test)]
mod tests {
    use belalang_comp::{compiler::Compiler, object::Object};
    use belalang_core::{lexer::Lexer, parser::Parser};

    use super::VM;

    #[test]
    fn constant() {
        let lexer = Lexer::new("5 + 10;".as_bytes());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        let mut compiler = Compiler::default();
        compiler.compile_program(program).unwrap();

        let mut vm = VM::new(compiler);
        vm.run().unwrap();

        assert_eq!(vm.stack.len(), 0);

        let Object::Integer(v) = vm.last_popped else {
            panic!()
        };
        assert_eq!(v, 15);
    }
}
