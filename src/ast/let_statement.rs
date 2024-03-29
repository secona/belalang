use crate::token;

use super::{Expression, Identifier, Node, Statement};

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        format!("let {} = {};", self.name.to_string(), self.value.to_string())
    }
}

impl Node for LetStatement {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{ast, token};

    use super::LetStatement;

    #[test]
    fn let_statement_to_string() {
        let stmt = LetStatement {
            token: token::Token::Let,
            name: ast::Identifier {
                token: token::Token::Ident(String::from("myVar")),
                value: String::from("myVar"),
            },
            value: Box::new(ast::Identifier {
                token: token::Token::Ident(String::from("anotherVar")),
                value: String::from("anotherVar"),
            })
        };

        assert_eq!(stmt.to_string(), "let myVar = anotherVar;");
    }
}
