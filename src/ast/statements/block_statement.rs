use super::Statement;
use crate::token;

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{{ {} }}", statements)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("fn() { 12; 14; 1 + 2; }");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::Expression);

        let f = testing::as_variant!(&expr.expression, ast::Expression::Function);

        assert_eq!(f.body.statements.len(), 3);

        // first statement
        let expr_0 =
            testing::as_variant!(&f.body.statements[0], ast::Statement::Expression);

        let int_0 = testing::as_variant!(&expr_0.expression, ast::Expression::Integer);

        assert_eq!(int_0.token, token::Token::Int("12".into()));
        assert_eq!(int_0.value, 12);

        // second statement
        let expr_1 =
            testing::as_variant!(&f.body.statements[1], ast::Statement::Expression);

        let int_1 = testing::as_variant!(&expr_1.expression, ast::Expression::Integer);

        assert_eq!(int_1.token, token::Token::Int("14".into()));
        assert_eq!(int_1.value, 14);

        // third statement
        let expr_2 =
            testing::as_variant!(&f.body.statements[2], ast::Statement::Expression);

        testing::expr_variant!(&expr_2.expression, Infix => (
            ast::Expression::Integer = 1,
            token::Token::Plus,
            ast::Expression::Integer = 2
        ));
    }
}
