use std::error::Error;

use belalang_comp::compiler::Compiler;
use belalang_comp::object::Object;
use belalang_comp::code;

pub struct VM {
    pub constants: Vec<Object>,
    pub instructions: Vec<u8>,

    pub stack: Vec<Object>,
    pub sp: usize,
}

impl VM {
    pub fn new(compiler: Compiler) -> Self {
        Self {
            constants: compiler.constants,
            instructions: compiler.instructions,

            stack: Vec::new(),
            sp: 0,
        }
    }

    pub fn stack_top(&mut self) -> Option<&Object> {
        self.stack.last()
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ip = 0;
        while ip < self.instructions.len() {
            match self.instructions[ip] {
                code::CONSTANT => {
                    let hi = self.instructions[ip + 1];
                    let lo = self.instructions[ip + 2];
                    let index = ((hi as u16) << 8) | (lo as u16);

                    println!("{} {} {}", hi, lo, index);

                    let object = self.constants[index as usize].clone();
                    self.push(object)?;

                    ip += 2;
                },
                _ => {}
            };

            ip += 1;
        }

        Ok(())
    }

    fn push(&mut self, object: Object) -> Result<(), Box<dyn Error>> {
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

        assert_eq!(vm.stack.len(), 2);

        println!("{:?}", vm.stack);

        // FIXME: supposed to be 3

        let Object::Integer(v) = vm.stack[0];
        assert_eq!(v, 5);

        let Object::Integer(v) = vm.stack[1];
        assert_eq!(v, 10);
    }
}
