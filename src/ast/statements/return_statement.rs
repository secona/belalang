use crate::ast::{Expression, Node, Statement};
use crate::token;

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
    use crate::{ast, test_util, token};

    use super::ReturnStatement;

    #[test]
    fn to_string() {
        let tests = [
            test_util::ToStringTest {
                obj: ReturnStatement {
                    token: token::Token::Return, 
                    return_value: Box::new(ast::Identifier {
                        token: token::Token::Ident(String::from("x")),
                        value: String::from("x"),
                    }),
                },
                exp: String::from("return x;"),
            },
            test_util::ToStringTest {
                obj: ReturnStatement {
                    token: token::Token::Return,
                    return_value: Box::new(ast::IntegerLiteral {
                        token: token::Token::Int(String::from("5")),
                        value: 5,
                    }),
                },
                exp: String::from("return 5;"),
            },
        ];

        tests.map(|t| t.test());
    }
}
