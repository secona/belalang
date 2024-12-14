use crate::error::SyntaxError;
use crate::token::Token;
use crate::utils::{digits, hex_byte_to_u8, letters, unwrap_or_return};

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

    pub fn next_token(&mut self) -> Result<Token, SyntaxError> {
        loop {
            match self.read_char() {
                // skips all lines that start with `#`
                Some(b'#') => {
                    while let Some(ch) = self.read_char() {
                        if ch == b'\n' {
                            break;
                        }
                    }
                }
                // skips all empty whitespaces
                Some(b' ' | b'\t' | b'\n' | b'\r') => (),
                // early return if it reached the EOF
                None => return Ok(Token::EOF),
                // break the loop if it isn't a whitespace or a comment
                _ => break,
            };
        }

        match self.ch {
            b':' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::ColonAssign)
                }
                _ => Err(SyntaxError::UnknownToken(":".into())),
            },
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
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::BitAndAssign)
                }
                _ => Ok(Token::BitAnd),
            },
            b'|' => match self.peek_char() {
                Some(b'|') => {
                    self.read_char();
                    Ok(Token::Or)
                }
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::BitOrAssign)
                }
                _ => Ok(Token::BitOr),
            },
            b'^' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::BitXorAssign)
                }
                _ => Ok(Token::BitXor),
            },
            b'<' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::Le)
                }
                Some(b'<') => {
                    self.read_char();
                    match self.peek_char() {
                        Some(b'=') => {
                            self.read_char();
                            Ok(Token::ShiftLeftAssign)
                        }
                        _ => Ok(Token::ShiftLeft),
                    }
                }
                _ => Ok(Token::Lt),
            },
            b'>' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Ok(Token::Ge)
                }
                Some(b'>') => {
                    self.read_char();
                    match self.peek_char() {
                        Some(b'=') => {
                            self.read_char();
                            Ok(Token::ShiftRightAssign)
                        }
                        _ => Ok(Token::ShiftRight),
                    }
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
            _ => Err(SyntaxError::UnknownToken(
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

    pub fn read_string(&mut self) -> Result<Token, SyntaxError> {
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
                            unwrap_or_return!(self.read_char(), Err(SyntaxError::UnexpectedEOF));
                        let lo_c =
                            unwrap_or_return!(self.read_char(), Err(SyntaxError::UnexpectedEOF));

                        let hi = unwrap_or_return!(
                            hex_byte_to_u8(hi_c),
                            Err(SyntaxError::UnknownEscapeString(
                                String::from_utf8(vec![b'x', hi_c, lo_c]).unwrap(),
                            ))
                        );

                        let lo = unwrap_or_return!(
                            hex_byte_to_u8(lo_c),
                            Err(SyntaxError::UnknownEscapeString(
                                String::from_utf8(vec![b'x', hi_c, lo_c]).unwrap(),
                            ))
                        );

                        result.push((hi << 4) | lo);
                    }
                    Some(c) => {
                        return Err(SyntaxError::UnknownEscapeString(
                            String::from_utf8(vec![c]).unwrap(),
                        ))
                    }
                    None => return Err(SyntaxError::UnclosedString()),
                },
                Some(b'"') => break,
                Some(c) => result.push(c),
                None => return Err(SyntaxError::UnclosedString()),
            }
        }

        Ok(Token::String(String::from_utf8(result).unwrap()))
    }

    pub fn read_identifier(&mut self) -> Result<Token, SyntaxError> {
        let position = self.position;

        while matches!(self.peek_char(), Some(letters!() | digits!())) {
            self.read_char();
        }

        Ok(Token::from(&self.input[position..self.read_position]))
    }

    pub fn read_number(&mut self) -> Result<Token, SyntaxError> {
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
