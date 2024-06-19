use crate::{
    error::ParserError,
    token::Token,
    unwrap_or_return,
    utils::hex_byte_to_u8,
};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        if self.skip_whitespace_and_comments().is_err() {
            return Ok(Token::EOF);
        }

        let tok: Token = match self.ch {
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
                    return Err(ParserError::UnknownToken(
                        String::from_utf8(vec![self.ch]).unwrap(),
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
            b'"' => self.read_string()?,
            _ => {
                if self.is_letter() {
                    self.read_identifier()
                } else if self.is_digit() {
                    self.read_number()
                } else {
                    return Err(ParserError::UnknownToken(
                        String::from_utf8(vec![self.ch]).unwrap(),
                    ));
                }
            }
        };

        Ok(tok)
    }

    pub fn read_char(&mut self) -> Option<u8> {
        self.ch = self.peek_char()?;

        self.position = self.read_position;
        self.read_position += 1;

        Some(self.ch)
    }

    pub fn peek_char(&self) -> Option<u8> {
        self.input.get(self.read_position).copied()
    }

    /// Return error if encounters the EOF.
    pub fn skip_whitespace_and_comments(&mut self) -> Result<(), ()> {
        loop {
            match self.read_char() {
                Some(b' ' | b'\t' | b'\n' | b'\r') => (),
                Some(b'#') => self.skip_comment(),
                None => return Err(()),
                _ => return Ok(()),
            };
        }
    }

    pub fn skip_comment(&mut self) {
        while let b'#' = self.ch {
            while let Some(ch) = self.read_char() {
                if ch == b'\n' {
                    break;
                }
            }
        }
    }

    pub fn read_string(&mut self) -> Result<Token, ParserError> {
        let mut result = Vec::<u8>::new();

        loop {
            match self.read_char() {
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
                    Some(b'\\') => {
                        self.read_char();
                        result.push(b'\\');
                    }
                    Some(b'x') => {
                        self.read_char(); // consume the 'x'

                        let hi_c =
                            unwrap_or_return!(self.read_char(), Err(ParserError::UnexpectedEOF));
                        let lo_c =
                            unwrap_or_return!(self.read_char(), Err(ParserError::UnexpectedEOF));

                        let hi = unwrap_or_return!(
                            hex_byte_to_u8(hi_c),
                            Err(ParserError::UnknownEscapeString(
                                String::from_utf8(vec![b'x', hi_c, lo_c]).unwrap(),
                            ))
                        );

                        let lo = unwrap_or_return!(
                            hex_byte_to_u8(lo_c),
                            Err(ParserError::UnknownEscapeString(
                                String::from_utf8(vec![b'x', hi_c, lo_c]).unwrap(),
                            ))
                        );

                        result.push((hi << 4) | lo);
                    }
                    Some(c) => {
                        return Err(ParserError::UnknownEscapeString(
                            String::from_utf8(vec![c]).unwrap(),
                        ))
                    }
                    None => return Err(ParserError::UnclosedString()),
                },
                Some(b'"') => break,
                Some(c) => result.push(c),
                None => return Err(ParserError::UnclosedString()),
            }
        }

        Ok(Token::String(String::from_utf8(result).unwrap()))
    }

    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while self.peek_is_letter() {
            self.read_char();
        }

        Token::from(&self.input[position..self.read_position])
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.peek_is_digit() {
            self.read_char();
        }

        let num = &self.input[position..self.read_position];
        let num = std::str::from_utf8(num).unwrap();
        Token::Int(String::from(num))
    }

    pub fn is_letter(&self) -> bool {
        match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
            _ => false,
        }
    }

    pub fn peek_is_letter(&self) -> bool {
        match self.peek_char() {
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => true,
            _ => false,
        }
    }

    pub fn is_digit(&self) -> bool {
        match self.ch {
            b'0'..=b'9' => true,
            _ => false,
        }
    }

    pub fn peek_is_digit(&self) -> bool {
        match self.peek_char() {
            Some(b'0'..=b'9') => true,
            _ => false,
        }
    }
}
