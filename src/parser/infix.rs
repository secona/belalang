#![allow(dead_code)]

use crate::{
    ast::{self, Expression},
    token,
};

use super::Precedence;

impl super::Parser<'_> {
    pub fn infix_fn(
        &mut self,
        tok: &token::Token,
        left: Expression,
    ) -> Result<Expression, Expression> {
        self.next_token();

        match tok {
            token::Token::Plus
            | token::Token::Minus
            | token::Token::Slash
            | token::Token::Asterisk
            | token::Token::Percent
            | token::Token::Eq
            | token::Token::NotEq
            | token::Token::GT
            | token::Token::LT => self.parse_infix_expression(left),
            token::Token::LParen => self.parse_call_expression(left),
            _ => Err(left),
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, Expression> {
        let token = self.curr_token.clone();
        let operator = self.curr_token.clone();
        let precedence = self.curr_precedence();

        self.next_token();

        let right = self.parse_expression(precedence).unwrap();

        Ok(Expression::InfixExpression(ast::InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, Expression> {
        if let Some(args) = self.parse_call_args() {
            Ok(Expression::CallExpression(ast::CallExpression {
                token: self.curr_token.clone(),
                function: Box::new(function),
                args,
            }))
        } else {
            Err(function)
        }
    }

    fn parse_call_args(&mut self) -> Option<Vec<Expression>> {
        let mut args = Vec::new();

        if self.peek_token_is(token::Token::RParen) {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(token::Token::Comma) {
            self.next_token();
            self.next_token();

            args.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek(token::Token::RParen) {
            return None;
        }

        Some(args)
    }
}
