use crate::{
    ast::{self, Expression},
    token,
};

use super::Precedence;

impl super::Parser<'_> {
    pub fn prefix_fn(&mut self) -> Option<Expression> {
        match self.curr_token {
            token::Token::Ident(_) => Some(self.parse_identifier()),
            token::Token::Int(_) => self.parse_integer_literal(),
            token::Token::Bang | token::Token::Minus => self.parse_prefix_expression(),
            token::Token::True | token::Token::False => self.parse_boolean(),
            token::Token::LParen => self.parse_grouped_expression(),
            token::Token::If => self.parse_if_expression(),
            token::Token::Function => self.parse_function_literal(),
            token::Token::String(_) => self.parse_string_literal(),
            _ => {
                self.errors.push(format!(
                    "no prefix parse function for {} found.",
                    self.curr_token.to_string()
                ));
                None
            }
        }
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.clone().to_string(),
        })
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let literal = self.curr_token.clone().to_string();

        match literal.parse::<i64>() {
            Ok(lit) => Some(Expression::IntegerLiteral(ast::IntegerLiteral {
                token: self.curr_token.clone(),
                value: lit,
            })),
            Err(_) => {
                self.errors
                    .push(format!("could not parse {} as integer.", literal));
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prev_token = self.curr_token.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix).unwrap();

        Some(Expression::PrefixExpression(ast::PrefixExpression {
            operator: prev_token.clone(),
            token: prev_token,
            right: Box::new(right),
        }))
    }

    fn parse_boolean(&self) -> Option<Expression> {
        Some(Expression::BooleanExpression(ast::BooleanExpression {
            token: self.curr_token.clone(),
            value: self.curr_token_is(token::Token::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(token::Token::RParen) {
            None
        } else {
            expr
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();

        if !self.expect_peek(token::Token::LParen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest).unwrap();

        if !self.expect_peek(token::Token::RParen) {
            return None;
        }

        if !self.expect_peek(token::Token::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token_is(token::Token::Else) {
            self.next_token();

            if !self.expect_peek(token::Token::LBrace) {
                return None;
            }

            Some(self.parse_block_statement())
        } else {
            None
        };

        Some(Expression::IfExpression(ast::IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();

        if !self.expect_peek(token::Token::LParen) {
            return None;
        }

        let params = self.parse_function_params()?;

        if !self.expect_peek(token::Token::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FunctionLiteral(ast::FunctionLiteral {
            token,
            params,
            body,
        }))
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(ast::StringLiteral {
            token: self.curr_token.clone(),
            value: self.curr_token.to_string(),
        }))
    }

    fn parse_function_params(&mut self) -> Option<Vec<ast::Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(token::Token::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();
        identifiers.push(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.to_string(),
        });

        while self.peek_token_is(token::Token::Comma) {
            self.next_token();
            self.next_token();

            identifiers.push(ast::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.to_string(),
            });
        }

        if !self.expect_peek(token::Token::RParen) {
            return None;
        }

        Some(identifiers)
    }
}
