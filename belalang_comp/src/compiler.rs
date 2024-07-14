use belalang_core::ast::{BlockExpression, Expression, Program, Statement};
use belalang_core::token::Token;

use crate::code;
use crate::error::CompileError;
use crate::object::{Function, Object};
use crate::scope::{ScopeLevel, ScopeManager};

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
                    let symbol = self.scope.define(var.name.value)?;
                    let scope = symbol.scope;
                    let index = symbol.index;

                    self.compile_expression(*var.value)?;

                    self.set_variable(&scope, index);
                }
                Token::Assign
                | Token::AddAssign
                | Token::SubAssign
                | Token::MulAssign
                | Token::DivAssign
                | Token::ModAssign => {
                    let symbol = self.scope.resolve(var.name.value)?;
                    let scope = symbol.scope;
                    let index = symbol.index;

                    match var.token {
                        Token::Assign => {
                            self.compile_expression(*var.value)?;
                        }
                        Token::AddAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(code::ADD);
                        }
                        Token::SubAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(code::SUB);
                        }
                        Token::MulAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(code::MUL);
                        }
                        Token::DivAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(code::DIV);
                        }
                        Token::ModAssign => {
                            self.get_variable(&scope, index);
                            self.compile_expression(*var.value)?;
                            self.add_bytecode(code::MOD);
                        }
                        _ => unreachable!(),
                    }

                    self.set_variable(&scope, index);
                }
                _ => todo!(),
            },

            Expression::Call(call) => {
                for arg in call.args.into_iter().rev() {
                    self.compile_expression(arg)?;
                }

                self.compile_expression(*call.function)?;
                self.add_bytecode(code::CALL);
            }

            Expression::Index(_) => todo!(),

            Expression::Function(mut function) => {
                self.scope.enter();

                let arity = function.params.len();

                for param in function.params.drain(..) {
                    self.scope.define(param.value)?;
                }

                for statement in function.body.statements {
                    self.compile_statement(statement)?;
                }

                let scope = self.scope.leave();
                let mut instructions = scope.instructions;

                match instructions.last() {
                    Some(&code::RETURN_VALUE) => (),
                    _ => instructions.push(code::RETURN_VALUE),
                };

                let index = self.add_constant(Object::Function(Function {
                    arity,
                    instructions,
                })) as u16;

                self.add_instruction(code::constant(index).to_vec());
            }

            Expression::Identifier(ident) => {
                let symbol = self.scope.resolve(ident.value)?;
                let scope = symbol.scope;
                let index = symbol.index;

                self.get_variable(&scope, index);
            }

            Expression::If(r#if) => {
                self.compile_expression(*r#if.condition)?;

                let jif = self.add_instruction(code::jump_if_false(0).to_vec());
                let jif_index = self.scope.current().instructions.len();

                self.scope.enter();
                self.compile_block(r#if.consequence)?;
                let scope = self.scope.leave();

                self.scope
                    .current_mut()
                    .instructions
                    .extend(scope.instructions);

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
                            self.scope.enter();
                            self.compile_block(block)?;
                            let scope = self.scope.leave();

                            self.scope
                                .current_mut()
                                .instructions
                                .extend(scope.instructions);
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
                self.scope.enter();
                self.compile_block(block)?;
                let scope = self.scope.leave();

                self.scope
                    .current_mut()
                    .instructions
                    .extend(scope.instructions);
            }
        };

        Ok(())
    }

    fn compile_block(&mut self, block: BlockExpression) -> Result<(), CompileError> {
        for statement in block.statements {
            self.compile_statement(statement)?;
        }

        Ok(())
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

    fn get_variable(&mut self, scope: &ScopeLevel, index: usize) -> usize {
        match scope {
            ScopeLevel::Global => self.add_instruction(code::get_global(index as u16).to_vec()),
            ScopeLevel::Local => self.add_instruction(code::get_local(index as u8).to_vec()),
            ScopeLevel::Builtin => self.add_instruction(code::get_builtin(index as u8).to_vec()),
        }
    }

    fn set_variable(&mut self, scope: &ScopeLevel, index: usize) -> usize {
        match scope {
            ScopeLevel::Global => self.add_instruction(code::set_global(index as u16).to_vec()),
            ScopeLevel::Local => self.add_instruction(code::set_local(index as u8).to_vec()),
            ScopeLevel::Builtin => 0,
        }
    }
}
