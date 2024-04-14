use crate::{ast, token};

use super::Precedence;

impl super::Parser {
    pub fn prefix_fn(&mut self) -> Option<Box<dyn ast::Expression>> {
        match self.curr_token {
            token::Token::Ident(_) => Some(self.parse_identifier()),
            token::Token::Int(_) => self.parse_integer_literal(),
            token::Token::Bang | token::Token::Minus => self.parse_prefix_expression(),
            token::Token::True | token::Token::False => self.parse_boolean(),
            token::Token::LParen => self.parse_grouped_expression(),
            token::Token::If => self.parse_if_expression(),
            token::Token::Function => self.parse_function_literal(),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Box<dyn ast::Expression> {
        Box::new(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.clone().to_string(),
        })
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        let literal = self.curr_token.clone().to_string();

        match literal.parse::<i64>() {
            Ok(lit) => Some(Box::new(ast::IntegerLiteral {
                token: self.curr_token.clone(),
                value: lit,
            })),
            Err(_) => {
                self.errors
                    .push(format!("could not parse {} as integer", literal).to_owned());
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        let prev_token = self.curr_token.clone();

        self.next_token();

        Some(Box::new(ast::PrefixExpression {
            operator: prev_token.clone().to_string(),
            token: prev_token,
            right: self.parse_expression(Precedence::Prefix).unwrap(),
        }))
    }

    fn parse_boolean(&self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::BooleanExpression {
            token: self.curr_token.clone(),
            value: self.curr_token_is(token::Token::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(token::Token::RParen) {
            None
        } else {
            expr
        }
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
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

        Some(Box::new(ast::IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }))
    }

    fn parse_function_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        let token = self.curr_token.clone();

        if !self.expect_peek(token::Token::LParen) {
            return None;
        }

        let params = self.parse_function_params()?;

        if !self.expect_peek(token::Token::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Box::new(ast::FunctionLiteral {
            token,
            params,
            body,
        }))
    }

    fn parse_function_params(&mut self) -> Option<Vec<Box<ast::Identifier>>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(token::Token::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();
        identifiers.push(Box::new(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.to_string(),
        }));

        while self.peek_token_is(token::Token::Comma) {
            self.next_token();
            self.next_token();

            identifiers.push(Box::new(ast::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.to_string(),
            }));
        }

        if !self.expect_peek(token::Token::RParen) {
            return None
        }

        Some(identifiers)
    }
}
