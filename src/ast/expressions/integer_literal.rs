use crate::token;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("12;");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::Expression);

        let int = testing::as_variant!(&expr.expression, ast::Expression::Integer);

        assert_eq!(int.token, token::Token::Int("12".into()));
        assert_eq!(int.value, 12);
    }
}
