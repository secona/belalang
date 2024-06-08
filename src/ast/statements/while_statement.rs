use crate::{ast::Expression, token};

use super::BlockStatement;

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub block: BlockStatement,
}

impl std::fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while ({}) {}", self.condition, self.block)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing};

    #[test]
    fn parsing() {
        let program = testing::test_parse("while (true) { 12; }");

        assert_eq!(program.statements.len(), 1);

        let stmt = testing::as_variant!(&program.statements[0], ast::Statement::WhileStatement);

        testing::expr_variant!(&*stmt.condition, ast::Expression::BooleanExpression = true);

        assert_eq!(stmt.block.statements.len(), 1);

        let expr_0 = testing::as_variant!(
            &stmt.block.statements[0],
            ast::Statement::ExpressionStatement
        );

        testing::expr_variant!(&expr_0.expression, ast::Expression::IntegerLiteral = 12);
    }
}
