use crate::token;
use crate::ast::{Expression, Node, Statement};

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Box<dyn Expression>,
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        format!("return {};", self.return_value.to_string())
    }
}

impl Node for ReturnStatement {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{ast, token};

    use super::ReturnStatement;

    #[test]
    fn return_statement_to_string() {
        let stmt = ReturnStatement {
            token: token::Token::Return,
            return_value: Box::new(ast::Identifier {
                token: token::Token::Int(String::from("5")),
                value: String::from("5"),
            }),
        };

        assert_eq!(stmt.to_string(), "return 5;");
    }
}
