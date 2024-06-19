use crate::{error::ParserError, token::Token};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<&'a u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        self.skip_whitespace_and_comments();

        let tok: Token = match self.ch {
            None => Token::EOF,
            Some(ch) => match ch {
                b'=' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Eq
                    }
                    _ => Token::Assign,
                },
                b'!' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Ne
                    }
                    _ => Token::Not,
                },
                b':' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::ColonAssign
                    }
                    _ => {
                        return Err(ParserError::IllegalToken(
                            String::from_utf8(vec![*ch]).unwrap(),
                        ));
                    }
                },
                b'<' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Le
                    }
                    _ => Token::Lt,
                },
                b'>' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::Ge
                    }
                    _ => Token::Gt,
                },
                b'+' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::AddAssign
                    }
                    _ => Token::Add,
                },
                b'-' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::SubAssign
                    }
                    _ => Token::Sub,
                },
                b'*' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::MulAssign
                    }
                    _ => Token::Mul,
                },
                b'/' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::DivAssign
                    }
                    _ => Token::Div,
                },
                b'%' => match self.peek_char() {
                    Some(b'=') => {
                        self.read_char();
                        Token::ModAssign
                    }
                    _ => Token::Mod,
                },
                b'(' => Token::LeftParen,
                b')' => Token::RightParen,
                b'{' => Token::LeftBrace,
                b'}' => Token::RightBrace,
                b';' => Token::Semicolon,
                b',' => Token::Comma,
                b'\\' => Token::Backslash,
                b'"' => self.read_string(),
                _ => {
                    if self.is_letter() {
                        return Ok(self.read_identifier());
                    } else if self.is_digit() {
                        return Ok(self.read_number());
                    } else {
                        return Err(ParserError::IllegalToken(
                            String::from_utf8(vec![*ch]).unwrap(),
                        ));
                    }
                }
            },
        };

        self.read_char();
        Ok(tok)
    }

    pub fn read_char(&mut self) -> Option<&'a u8> {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(&self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.ch
    }

    pub fn peek_char(&self) -> Option<&'a u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(&self.input[self.read_position])
        }
    }

    pub fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.ch {
                Some(b'#') => self.skip_comment(),
                Some(b' ' | b'\t' | b'\n' | b'\r') => {
                    self.read_char();
                }
                _ => break,
            };
        }
    }

    pub fn skip_comment(&mut self) {
        while let Some(b'#') = self.ch {
            loop {
                match self.ch {
                    Some(b'\n') | None => {
                        self.read_char();
                        break;
                    }
                    _ => self.read_char(),
                };
            }
        }
    }

    pub fn read_string(&mut self) -> Token {
        let mut result = Vec::<u8>::new();

        loop {
            self.read_char();
            match self.ch {
                Some(b'\\') => match self.peek_char() {
                    Some(b'n') => {
                        self.read_char();
                        result.push(b'\n');
                    }
                    Some(b'r') => {
                        self.read_char();
                        result.push(b'\r');
                    }
                    Some(b't') => {
                        self.read_char();
                        result.push(b'\t');
                    }
                    Some(b'"') => {
                        self.read_char();
                        result.push(b'"');
                    }
                    _ => result.push(b'\\'),
                }
                Some(b'"' | 0) | None => break,
                Some(c) => result.push(*c),
            }
        }

        Token::String(String::from_utf8(result).unwrap())
    }

    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while self.is_letter() {
            self.read_char();
        }

        Token::from(&self.input[position..self.position])
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.is_digit() {
            self.read_char();
        }

        let num = &self.input[position..self.position];
        let num = std::str::from_utf8(num).unwrap();
        Token::Int(String::from(num))
    }

    pub fn is_letter(&self) -> bool {
        if let Some(ch) = self.ch {
            *ch >= b'a' && *ch <= b'z' || *ch >= b'A' && *ch <= b'Z' || *ch == b'_'
        } else {
            false
        }
    }

    pub fn is_digit(&self) -> bool {
        match self.ch {
            Some(ch) => *ch >= b'0' && *ch <= b'9',
            None => false,
        }
    }
}
