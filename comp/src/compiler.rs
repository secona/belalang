use belalang_core::ast::{Expression, Node, Program, Statement};

pub struct Compiler {
    pub instructions: Vec<u8>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

impl Compiler {
    pub fn compile(&mut self, node: Node) {
        match node {
            Node::Program(program) => self.compile_program(program),
            Node::Expression(expression) => self.compile_expression(expression),
            Node::Statement(statement) => self.compile_statement(statement),
        }
    }

    pub fn compile_program(&mut self, program: Program) {
        for statement in program.statements {
            self.compile_statement(statement);
        }
    }

    pub fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expression(statement) => {
                self.compile_expression(statement.expression)
            },
            Statement::Return(_) => todo!(),
            Statement::While(_) => todo!(),
        }
    }

    pub fn compile_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Boolean(_) => todo!(),
            Expression::Integer(_) => todo!(),
            Expression::Float(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Null(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Var(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::Function(_) => todo!(),
            Expression::Identifier(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Infix(_) => todo!(),
            Expression::Prefix(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
