use super::Expression;
use crate::token;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub operator: token::Token,
    pub right: Box<Expression>,
}

impl std::fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.left.to_string(),
            self.operator,
            self.right.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("1 + 2;");

        assert_eq!(program.statements.len(), 1);

        let expr = testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        testing::expr_variant!(&expr.expression, Infix => (
            ast::Expression::IntegerLiteral = 1,
            token::Token::Plus,
            ast::Expression::IntegerLiteral = 2
        ));
    }
}
