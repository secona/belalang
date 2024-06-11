use crate::token;

#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub token: token::Token,
    pub value: bool,
}

impl std::fmt::Display for BooleanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing_true() {
        let program = testing::test_parse("true;");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::Expression);

        let bool_expr = testing::as_variant!(&expr.expression, ast::Expression::Boolean);

        assert_eq!(bool_expr.value, true);
        assert_eq!(bool_expr.token, token::Token::True);
    }

    #[test]
    fn parsing_false() {
        let program = testing::test_parse("false;");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::Expression);

        let bool_expr = testing::as_variant!(&expr.expression, ast::Expression::Boolean);

        assert_eq!(bool_expr.value, false);
        assert_eq!(bool_expr.token, token::Token::False);
    }
}
