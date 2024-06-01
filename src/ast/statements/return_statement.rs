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

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    use super::ReturnStatement;

    #[test]
    fn to_string() {
        testing::stringify!(
            ReturnStatement {
                token: token::Token::Return,
                return_value: ast::Expression::Identifier(ast::expressions::Identifier {
                    token: token::Token::Ident(String::from("x")),
                    value: String::from("x"),
                }),
            },
            String::from("return x;")
        );

        testing::stringify!(
            ReturnStatement {
                token: token::Token::Return,
                return_value: ast::Expression::IntegerLiteral(ast::expressions::IntegerLiteral {
                    token: token::Token::Int(String::from("5")),
                    value: 5,
                }),
            },
            String::from("return 5;")
        );
    }
}
