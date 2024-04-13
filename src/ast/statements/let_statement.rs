use crate::ast::{Expression, Identifier, Node, Statement};
use crate::token;

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        format!(
            "let {} = {};",
            self.name.to_string(),
            self.value.to_string()
        )
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
    use crate::{ast, lexer, parser, test_util, token};

    use super::LetStatement;

    #[test]
    fn to_string() {
        let tests = [
            test_util::ToStringTest {
                obj: LetStatement {
                    token: token::Token::Let,
                    name: ast::Identifier {
                        token: token::Token::Ident(String::from("x")),
                        value: String::from("x"),
                    },
                    value: Box::new(ast::IntegerLiteral {
                        token: token::Token::Int(String::from("5")),
                        value: 5,
                    }),
                },
                exp: String::from("let x = 5;"),
            },
            test_util::ToStringTest {
                obj: LetStatement {
                    token: token::Token::Let,
                    name: ast::Identifier {
                        token: token::Token::Ident(String::from("myVar")),
                        value: String::from("myVar"),
                    },
                    value: Box::new(ast::Identifier {
                        token: token::Token::Ident(String::from("anotherVar")),
                        value: String::from("anotherVar"),
                    }),
                },
                exp: String::from("let myVar = anotherVar;"),
            },
        ];

        tests.map(|t| t.test());
    }

    #[test]
    fn to_struct() {
        let input = "let x = 5;".to_owned().into_bytes().into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::LetStatement>()
            .expect("not a(n) ast::LetStatement");

        assert_eq!(stmt.name.token, token::Token::Ident(String::from("x")));
        assert_eq!(stmt.name.value, String::from("x"));

        let int_lit = stmt
            .value
            .downcast_ref::<ast::IntegerLiteral>()
            .expect("not a(n) ast::IntegerLiteral");

        assert_eq!(int_lit.value, 5);
        assert_eq!(int_lit.token, token::Token::Int(String::from("5")));
    }
}
