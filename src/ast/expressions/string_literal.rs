use crate::token;

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: token::Token,
    pub value: String,
}

impl std::fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("\"Hello, World!\"");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let s = testing::as_variant!(&expr.expression, ast::Expression::StringLiteral);

        assert_eq!(s.token, token::Token::String("Hello, World!".into()));
        assert_eq!(s.value, "Hello, World!");
    }
}
