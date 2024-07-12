use belalang_core::ast::{BlockExpression, Expression, Program, Statement};
use belalang_core::token::Token;

use crate::code;
use crate::error::CompileError;
use crate::object::{Function, Object};
use crate::scope::{CompilationScope, ScopeManager, SymbolScope};

pub struct Code {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}

#[derive(Default)]
pub struct Compiler {
    prev_constants: usize,
    pub constants: Vec<Object>,
    pub scope: ScopeManager,
}

impl Compiler {
    pub fn compile_program(&mut self, program: Program) -> Result<Code, CompileError> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }

        let code = Code {
            instructions: self.scope.main_scope.instructions.drain(..).collect(),
            constants: self.constants[self.prev_constants..].to_vec(),
        };

        self.prev_constants = self.constants.len();

        Ok(code)
    }

    pub fn compile_statement(&mut self, statement: Statement) -> Result<(), CompileError> {
        match statement {
            Statement::Expression(statement) => {
                self.compile_expression(statement.expression)?;
                self.add_bytecode(code::POP);
            }

            Statement::Return(r#return) => {
                self.compile_expression(r#return.return_value)?;
                self.add_bytecode(code::RETURN_VALUE);
            }

            Statement::While(_) => todo!(),
        };

        Ok(())
    }

    pub fn compile_expression(&mut self, expression: Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Boolean(boolean) => {
                self.add_bytecode(if boolean.value {
                    code::TRUE
                } else {
                    code::FALSE
                });
            }

            Expression::Integer(integer) => {
                let integer = Object::Integer(integer.value);
                let index = self.add_constant(integer) as u16;
                self.add_instruction(code::constant(index).to_vec());
            }

            Expression::Float(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Null(_) => todo!(),
            Expression::Array(_) => todo!(),

            Expression::Var(var) => match var.token {
                Token::ColonAssign => {
                    self.compile_expression(*var.value)?;

                    let symbol = self.scope.define(var.name.value)?;
                    let index = symbol.index;

                    match symbol.scope {
                        SymbolScope::Global => {
                            self.add_instruction(code::def_global(index as u16).to_vec());
                        }
                        SymbolScope::Local => {
                            self.add_instruction(code::set_local(index as u8).to_vec());
                        }
                    }
                }
                _ => todo!(),
            },

            Expression::Call(call) => {
                self.compile_expression(*call.function)?;
                self.add_bytecode(code::CALL);
            }

            Expression::Index(_) => todo!(),

            Expression::Function(function) => {
                let scope = self.compile_block(function.body)?;
                let mut instructions = scope.instructions;

                match instructions.last() {
                    Some(&code::RETURN_VALUE) => (),
                    _ => instructions.push(code::RETURN_VALUE),
                };

                let index = self.add_constant(Object::Function(Function {
                    instructions,
                    arity: scope.symbol_count,
                })) as u16;

                self.add_instruction(code::constant(index).to_vec());
            }

            Expression::Identifier(ident) => {
                let symbol = self.scope.resolve(ident.value)?;
                let index = symbol.index;

                self.add_instruction(if matches!(symbol.scope, SymbolScope::Global) {
                    code::get_global(index as u16).to_vec()
                } else {
                    code::get_local(index as u8).to_vec()
                });
            }

            Expression::If(r#if) => {
                self.compile_expression(*r#if.condition)?;

                let jif = self.add_instruction(code::jump_if_false(0).to_vec());
                let jif_index = self.scope.current().instructions.len();

                let mut scope = self.compile_block(r#if.consequence)?;
                self.scope
                    .current_mut()
                    .instructions
                    .append(&mut scope.instructions);

                let jump = self.add_instruction(code::jump(0).to_vec());
                let jump_index = self.scope.current().instructions.len();

                let post_consequence = self.scope.current().instructions.len();
                self.replace_u16_operand(jif, (post_consequence - jif_index) as u16);

                match r#if.alternative {
                    None => {
                        self.add_bytecode(code::NULL);
                    }
                    Some(alt) => match *alt {
                        Expression::Block(block) => {
                            let mut scope = self.compile_block(block)?;
                            self.scope
                                .current_mut()
                                .instructions
                                .append(&mut scope.instructions);
                        }
                        _ => {
                            self.compile_expression(*alt)?;
                        }
                    },
                };

                let post_alternative = self.scope.current().instructions.len();
                self.replace_u16_operand(jump, (post_alternative - jump_index) as u16);
            }

            Expression::Infix(infix) => {
                match infix.operator {
                    Token::Gt | Token::Ge => {
                        self.compile_expression(*infix.right)?;
                        self.compile_expression(*infix.left)?;
                    }
                    _ => {
                        self.compile_expression(*infix.left)?;
                        self.compile_expression(*infix.right)?;
                    }
                }

                self.add_bytecode(match infix.operator {
                    Token::Add => code::ADD,
                    Token::Sub => code::SUB,
                    Token::Mul => code::MUL,
                    Token::Div => code::DIV,
                    Token::Mod => code::MOD,
                    Token::Eq => code::EQUAL,
                    Token::Ne => code::NOT_EQUAL,
                    Token::Lt | Token::Gt => code::LESS_THAN,
                    Token::Le | Token::Ge => code::LESS_THAN_EQUAL,
                    _ => return Err(CompileError::UnknownInfixOp(infix.operator)),
                });
            }

            Expression::Prefix(prefix) => {
                self.compile_expression(*prefix.right)?;
                self.add_bytecode(match prefix.operator {
                    Token::Sub => code::MINUS,
                    Token::Not => code::BANG,
                    _ => return Err(CompileError::UnknownInfixOp(prefix.operator)),
                });
            }

            Expression::Block(block) => {
                let mut scope = self.compile_block(block)?;

                self.scope
                    .current_mut()
                    .instructions
                    .append(&mut scope.instructions);
            }
        };

        Ok(())
    }

    fn compile_block(&mut self, block: BlockExpression) -> Result<CompilationScope, CompileError> {
        self.scope.enter();

        for statement in block.statements {
            self.compile_statement(statement)?;
        }

        let mut scope = self.scope.leave();

        if let Some(&code::POP) = scope.instructions.last() {
            scope.instructions.pop();
        }

        Ok(scope)
    }

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    pub fn add_bytecode(&mut self, byte: u8) -> usize {
        let scope = self.scope.current_mut();

        scope.instructions.push(byte);
        scope.instructions.len() - 1
    }

    pub fn add_instruction(&mut self, bytes: Vec<u8>) -> usize {
        let pos = self.scope.current_mut().instructions.len();

        for byte in bytes {
            self.add_bytecode(byte);
        }

        pos
    }

    pub fn replace_u16_operand(&mut self, index: usize, value: u16) {
        let scope = self.scope.current_mut();

        scope.instructions[index + 1] = (value >> 8) as u8;
        scope.instructions[index + 2] = (value & 0xFF) as u8;
    }
}
