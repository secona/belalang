use crate::ast::expressions::Expression;
use crate::ast::Identifier;
use crate::token;

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("let x = 5;");

        assert_eq!(program.statements.len(), 1);

        let stmt = testing::as_variant!(&program.statements[0], ast::Statement::LetStatement);

        assert_eq!(stmt.token, token::Token::Let);
        assert_eq!(stmt.name.token, token::Token::Ident(String::from("x")));
        assert_eq!(stmt.name.value, String::from("x"));

        testing::expr_variant!(&stmt.value, ast::Expression::IntegerLiteral = 5);
    }
}
