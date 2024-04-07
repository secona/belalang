#![allow(dead_code)]

use crate::{ast, token};

use super::Precedence;

impl super::Parser {
    pub fn prefix_fn(&mut self) -> Option<Box<dyn ast::Expression>> {
        println!("{:?}", self.curr_token);
        match &self.curr_token.clone() {
            Some(tok) => match tok {
                token::Token::Ident(s) => Some(self.parse_identifier(&s)),
                token::Token::Int(s) => self.parse_integer_literal(&s),
                token::Token::Bang => self.parse_prefix_expression(&"!".to_owned()),
                token::Token::Minus => self.parse_prefix_expression(&"-".to_owned()),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_identifier(&self, literal: &String) -> Box<dyn ast::Expression> {
        Box::new(ast::Identifier {
            token: self.curr_token.clone().unwrap(),
            value: literal.clone(),
        })
    }

    fn parse_integer_literal(&mut self, literal: &String) -> Option<Box<dyn ast::Expression>> {
        match literal.parse::<i64>() {
            Ok(lit) => Some(Box::new(ast::IntegerLiteral {
                token: self.curr_token.clone().unwrap(),
                value: lit,
            })),
            Err(_) => {
                self.errors
                    .push(format!("could not parse {} as integer", literal).to_owned());
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self, literal: &String) -> Option<Box<dyn ast::Expression>> {
        let prev_token = self.curr_token.clone().unwrap();

        self.next_token();

        Some(Box::new(ast::PrefixExpression {
            token: prev_token,
            operator: literal.to_string(),
            right: self.parse_expression(Precedence::Prefix).unwrap(),
        }))
    }
}
