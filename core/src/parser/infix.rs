use crate::{
    ast::{self, Expression},
    error::ParserError,
    token::{arithmetic_tokens, comparison_tokens, Token},
};

use super::{expect_peek, Precedence};

impl super::Parser<'_> {
    pub fn parse_infix(&mut self, left: &Expression) -> Result<Option<Expression>, ParserError> {
        match self.peek_token {
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

            Token::ColonAssign | Token::Assign => {
                if !matches!(left, Expression::Identifier(_)) {
                    return Err(ParserError::InvalidLHS(left.clone()))
                }

                let name = ast::Identifier {
                    token: self.curr_token.clone(),
                    value: self.curr_token.to_string(),
                };

                self.next_token()?;
                let token = self.curr_token.clone();

                self.next_token()?;
                let value = Box::new(self.parse_expression(Precedence::Lowest)?);

                Ok(Some(Expression::Var(ast::VarExpression {
                    token,
                    name,
                    value,
                })))
            }

            Token::AddAssign
            | Token::SubAssign
            | Token::MulAssign
            | Token::DivAssign
            | Token::ModAssign => {
                if !matches!(left, Expression::Identifier(_)) {
                    return Err(ParserError::InvalidLHS(left.clone()))
                }

                let name = ast::Identifier {
                    token: self.curr_token.clone(),
                    value: self.curr_token.to_string(),
                };

                self.next_token()?;
                let token = self.curr_token.clone();

                self.next_token()?;
                let value = self.parse_expression(Precedence::Lowest)?;

                // probably need to change this monstrosity.
                Ok(Some(Expression::Var(ast::VarExpression {
                    token: Token::Assign,
                    name: name.clone(),
                    value: Box::new(Expression::Infix(ast::InfixExpression {
                        left: Box::new(Expression::Identifier(name)),
                        operator: match &token {
                            Token::AddAssign => Token::Add,
                            Token::SubAssign => Token::Sub,
                            Token::MulAssign => Token::Mul,
                            Token::DivAssign => Token::Div,
                            Token::ModAssign => Token::Mod,
                            _ => unreachable!(),
                        },
                        token,
                        right: Box::new(value),
                    })),
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
