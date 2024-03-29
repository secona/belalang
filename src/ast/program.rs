use crate::token;

use super::{Node, Statement};

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn add_stmt(&mut self, stmt: Box<dyn Statement>) {
        self.statements.push(stmt);
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for stmt in &self.statements {
            result.push_str(&stmt.to_string());
        }

        result
    }
}

impl Node for Program {
    fn token(&self) -> Option<&token::Token> {
        if self.statements.len() > 0 {
            self.statements[0].token()
        } else {
            None
        }
    }
}
