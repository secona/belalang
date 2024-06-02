use crate::ast::expressions::Expression;
use crate::token;

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ReturnStatement(value={})",
            self.return_value.to_string()
        ))
    }
}
