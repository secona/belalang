use crate::{
    ast::{BlockStatement, Expression, Node},
    token,
};

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl ToString for IfExpression {
    fn to_string(&self) -> String {
        let mut result = format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if let Some(stmt) = &self.alternative {
            result += &format!("else {}", stmt.to_string());
        }

        result
    }
}

impl Node for IfExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, token};

    #[test]
    fn works_without_else() {
        let input = "if (x < y) { x }"
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let if_expr = stmt
            .expression
            .downcast_ref::<ast::IfExpression>()
            .expect("not a(n) ast::IfExpression");

        assert!(if_expr.alternative.is_none());
        assert_eq!(if_expr.condition.to_string(), "(x < y)");
        assert_eq!(if_expr.token, token::Token::If);

        assert_eq!(
            if_expr.consequence.statements[0]
                .downcast_ref::<ast::ExpressionStatement>()
                .expect("not a(n) ast::ExpressionStatement")
                .to_string(),
            "x",
        );
    }

    #[test]
    fn works_with_else() {
        let input = "if (x < y) { x } else { y }"
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let if_expr = stmt
            .expression
            .downcast_ref::<ast::IfExpression>()
            .expect("not a(n) ast::IfExpression");

        assert_eq!(if_expr.token, token::Token::If);

        assert_eq!(if_expr.condition.to_string(), "(x < y)");

        assert_eq!(
            if_expr.consequence.statements[0]
                .downcast_ref::<ast::ExpressionStatement>()
                .expect("not a(n) ast::ExpressionStatement")
                .to_string(),
            "x",
        );

        let alt = if_expr.alternative.as_ref().expect("alternative is None");

        assert_eq!(alt.token, token::Token::LBrace);
        assert_eq!(
            alt.statements[0]
                .downcast_ref::<ast::ExpressionStatement>()
                .expect("not a(n) ast::ExpressionStatement")
                .to_string(),
            "y"
        );
    }

    #[test]
    fn works_multiple_statements() {
        let input = "if (x < y) { let a = 10; x }"
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let if_expr = stmt
            .expression
            .downcast_ref::<ast::IfExpression>()
            .expect("not a(n) ast::IfExpression");

        assert!(if_expr.alternative.is_none());
        assert_eq!(if_expr.condition.to_string(), "(x < y)");
        assert_eq!(if_expr.token, token::Token::If);

        assert_eq!(
            if_expr.consequence.statements[0]
                .downcast_ref::<ast::LetStatement>()
                .expect("not a(n) ast::LetStatement")
                .to_string(),
            "let a = 10",
        );

        assert_eq!(
            if_expr.consequence.statements[1]
                .downcast_ref::<ast::ExpressionStatement>()
                .expect("not a(n) ast::ExpressionStatement")
                .to_string(),
            "x",
        );
    }
}
