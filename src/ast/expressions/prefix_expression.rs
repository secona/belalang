use super::Expression;
use crate::token;

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: token::Token,
    pub right: Box<Expression>,
}

impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.right.to_string(),)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("-12");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let prefix = testing::as_variant!(&expr.expression, ast::Expression::PrefixExpression);

        assert_eq!(prefix.token, token::Token::Minus);
        assert_eq!(prefix.operator, token::Token::Minus);

        let right = testing::as_variant!(&*prefix.right, ast::Expression::IntegerLiteral);

        assert_eq!(right.token, token::Token::Int("12".into()));
        assert_eq!(right.value, 12);
    }
}
