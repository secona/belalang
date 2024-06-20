use crate::{
    ast::{self, Expression},
    error::ParserError,
    token::{arithmetic_tokens, comparison_tokens, Token},
};

use super::{expect_peek, Precedence};

impl super::Parser<'_> {
    pub fn parse_infix(
        &mut self,
        tok: &Token,
        left: &Expression,
    ) -> Result<Option<Expression>, ParserError> {
        match tok {
            // parse_infix: parse infix expression
            arithmetic_tokens!() | comparison_tokens!() | Token::Or | Token::And => {
                self.next_token()?;

                let token = self.curr_token.clone();
                let operator = self.curr_token.clone();
                let precedence = Precedence::from(&self.curr_token);

                self.next_token()?;

                let right = self.parse_expression(precedence)?;

                Ok(Some(Expression::Infix(ast::InfixExpression {
                    token,
                    left: Box::new(left.clone()),
                    operator,
                    right: Box::new(right),
                })))
            }

            // parse_call: parse call expression
            Token::LeftParen => {
                self.next_token()?;

                let args = self.parse_call_args()?;

                Ok(Some(Expression::Call(ast::CallExpression {
                    token: self.curr_token.clone(),
                    function: Box::new(left.clone()),
                    args,
                })))
            }

            _ => Ok(None),
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut args = Vec::new();

        if matches!(self.peek_token, Token::RightParen) {
            self.next_token()?;
            return Ok(args);
        }

        self.next_token()?;
        args.push(self.parse_expression(Precedence::Lowest)?);

        while matches!(self.peek_token, Token::Comma) {
            self.next_token()?;
            self.next_token()?;

            args.push(self.parse_expression(Precedence::Lowest)?);
        }

        expect_peek!(self, Token::RightParen);

        Ok(args)
    }
}
