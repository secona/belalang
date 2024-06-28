use crate::{
    error::ParserError,
    token::Token,
    utils::{digits, hex_byte_to_u8, letters, unwrap_or_return},
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
        if !self.skip_whitespace_and_comments() {
            return Ok(Token::EOF);
        }

        match self.ch {
            b'=' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::Eq)
                }
                _ => Ok(Token::Assign),
            },
            b'!' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::Ne)
                }
                _ => Ok(Token::Not),
            },
            b'&' => match self.peek_char() {
                Some(b'&') => {
                    self.read_char();
                    Ok(Token::And)
                }
                _ => Err(ParserError::UnknownToken(
                    String::from_utf8(vec![self.ch]).unwrap(),
                )),
            },
            b'|' => match self.peek_char() {
                Some(b'|') => {
                    self.read_char();
                    Ok(Token::Or)
                }
                _ => Err(ParserError::UnknownToken(
                    String::from_utf8(vec![self.ch]).unwrap(),
                )),
            },
            b':' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::ColonAssign)
                }
                _ => Err(ParserError::UnknownToken(
                    String::from_utf8(vec![self.ch]).unwrap(),
                )),
            },
            b'<' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::Le)
                }
                _ => Ok(Token::Lt),
            },
            b'>' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::Ge)
                }
                _ => Ok(Token::Gt),
            },
            b'+' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::AddAssign)
                }
                _ => Ok(Token::Add),
            },
            b'-' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::SubAssign)
                }
                _ => Ok(Token::Sub),
            },
            b'*' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::MulAssign)
                }
                _ => Ok(Token::Mul),
            },
            b'/' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::DivAssign)
                }
                _ => Ok(Token::Div),
            },
            b'%' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::ModAssign)
                }
                _ => Ok(Token::Mod),
            },
            b'(' => Ok(Token::LeftParen),
            b')' => Ok(Token::RightParen),
            b'{' => Ok(Token::LeftBrace),
            b'}' => Ok(Token::RightBrace),
            b'[' => Ok(Token::LeftBracket),
            b']' => Ok(Token::RightBracket),
            b';' => Ok(Token::Semicolon),
            b',' => Ok(Token::Comma),
            b'\\' => Ok(Token::Backslash),
            b'"' => self.read_string(),
            letters!() => Ok(self.read_identifier()?),
            digits!() => Ok(self.read_number()?),
            _ => Err(ParserError::UnknownToken(
                String::from_utf8(vec![self.ch]).unwrap(),
            )),
        }
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

    /// Return false if it encounters an EOF.
    pub fn skip_whitespace_and_comments(&mut self) -> bool {
        loop {
            match self.read_char() {
                Some(b' ' | b'\t' | b'\n' | b'\r') => (),
                Some(b'#') => self.skip_comment(),
                None => return false,
                _ => return true,
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

    pub fn read_identifier(&mut self) -> Result<Token, ParserError> {
        let position = self.position;

        while matches!(self.peek_char(), Some(letters!() | digits!())) {
            self.read_char();
        }

        Ok(Token::from(&self.input[position..self.read_position]))
    }

    pub fn read_number(&mut self) -> Result<Token, ParserError> {
        let mut has_decimal = false;
        let position = self.position;

        loop {
            match self.peek_char() {
                Some(digits!()) => {
                    self.read_char();
                }
                Some(b'.') if !has_decimal => {
                    has_decimal = true;
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let num = &self.input[position..self.read_position];
        let num = std::str::from_utf8(num).unwrap();

        Ok(if has_decimal {
            Token::Float(String::from(num))
        } else {
            Token::Int(String::from(num))
        })
    }
}
