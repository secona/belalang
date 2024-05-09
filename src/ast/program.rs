use super::Statement;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn add_stmt(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for stmt in &self.statements {
            result.push_str(&stmt.to_string());
        }

        f.write_str(&result)
    }
}
