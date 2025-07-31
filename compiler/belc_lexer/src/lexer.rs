use std::iter::Peekable;
use std::str::Chars;

use unicode_ident::{is_xid_continue, is_xid_start};

use super::Token;

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("unknown token: {0}")]
    UnknownToken(String),

    #[error("unknown escape string")]
    UnknownEscapeString,

    #[error("unclosed string")]
    UnclosedString,
}

pub fn char_to_u8(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

pub struct Lexer<'a> {
    current: Option<char>,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut chars = input.chars().peekable();
        let current = chars.next();

        Lexer { current, chars }
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.current;
        self.current = self.chars.next();
        result
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            match self.current {
                // skips all lines that start with `#`
                Some('#') => {
                    while let Some(c) = self.advance() {
                        if c == '\n' {
                            self.advance();
                            break;
                        }
                    }
                },
                // skips all empty whitespaces
                Some(' ' | '\t' | '\n' | '\r') => {
                    self.advance();
                },
                // break the loop if it isn't a whitespace or a comment
                _ => break,
            };
        }

        if self.current.is_none() {
            return Ok(Token::EOF);
        }

        match self.current {
            Some(':') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::ColonAssign)
                    },
                    _ => Err(LexerError::UnknownToken(":".into())),
                }
            },
            Some('=') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Eq)
                    },
                    _ => Ok(Token::Assign),
                }
            },
            Some('!') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Ne)
                    },
                    _ => Ok(Token::Not),
                }
            },
            Some('&') => {
                self.advance();
                match self.current {
                    Some('&') => {
                        self.advance();
                        Ok(Token::And)
                    },
                    Some('=') => {
                        self.advance();
                        Ok(Token::BitAndAssign)
                    },
                    _ => Ok(Token::BitAnd),
                }
            },
            Some('|') => {
                self.advance();
                match self.current {
                    Some('|') => {
                        self.advance();
                        Ok(Token::Or)
                    },
                    Some('=') => {
                        self.advance();
                        Ok(Token::BitOrAssign)
                    },
                    _ => Ok(Token::BitOr),
                }
            },
            Some('^') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::BitXorAssign)
                    },
                    _ => Ok(Token::BitXor),
                }
            },
            Some('<') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Le)
                    },
                    Some('<') => {
                        self.advance();
                        match self.chars.peek() {
                            Some('=') => {
                                self.advance();
                                Ok(Token::ShiftLeftAssign)
                            },
                            _ => Ok(Token::ShiftLeft),
                        }
                    },
                    _ => Ok(Token::Lt),
                }
            },
            Some('>') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Ge)
                    },
                    Some('>') => {
                        self.advance();
                        match self.chars.peek() {
                            Some('=') => {
                                self.advance();
                                Ok(Token::ShiftRightAssign)
                            },
                            _ => Ok(Token::ShiftRight),
                        }
                    },
                    _ => Ok(Token::Gt),
                }
            },
            Some('+') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::AddAssign)
                    },
                    _ => Ok(Token::Add),
                }
            },
            Some('-') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::SubAssign)
                    },
                    _ => Ok(Token::Sub),
                }
            },
            Some('*') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::MulAssign)
                    },
                    _ => Ok(Token::Mul),
                }
            },
            Some('/') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::DivAssign)
                    },
                    _ => Ok(Token::Div),
                }
            },
            Some('%') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::ModAssign)
                    },
                    _ => Ok(Token::Mod),
                }
            },
            Some('(') => {
                self.advance();
                Ok(Token::LeftParen)
            },
            Some(')') => {
                self.advance();
                Ok(Token::RightParen)
            },
            Some('{') => {
                self.advance();
                Ok(Token::LeftBrace)
            },
            Some('}') => {
                self.advance();
                Ok(Token::RightBrace)
            },
            Some('[') => {
                self.advance();
                Ok(Token::LeftBracket)
            },
            Some(']') => {
                self.advance();
                Ok(Token::RightBracket)
            },
            Some(';') => {
                self.advance();
                Ok(Token::Semicolon)
            },
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            },
            Some('\\') => {
                self.advance();
                Ok(Token::Backslash)
            },
            Some('"') => self.read_string(),
            Some(c) if c.is_numeric() => Ok(self.read_number()?),
            Some(_) => Ok(self.read_identifier()?),
            _ => unreachable!(),
        }
    }

    fn read_string(&mut self) -> Result<Token, LexerError> {
        self.advance(); // consume the opening "
        let mut result = String::new();

        loop {
            match self.advance() {
                Some('\\') => match self.current {
                    Some('n') => {
                        self.advance();
                        result.push('\n');
                    },
                    Some('r') => {
                        self.advance();
                        result.push('\r');
                    },
                    Some('t') => {
                        self.advance();
                        result.push('\t');
                    },
                    Some('"') => {
                        self.advance();
                        result.push('"');
                    },
                    Some('\\') => {
                        self.advance();
                        result.push('\\');
                    },
                    Some('x') => {
                        self.advance(); // consume the 'x'

                        match (self.advance().and_then(char_to_u8), self.advance().and_then(char_to_u8)) {
                            (Some(hi), Some(lo)) => result.push(((hi << 4) | lo) as char),
                            (_, _) => return Err(LexerError::UnknownEscapeString),
                        }
                    },
                    Some(_) => return Err(LexerError::UnknownEscapeString),
                    None => return Err(LexerError::UnclosedString),
                },
                Some('"') => break,
                Some(c) => result.push(c),
                None => return Err(LexerError::UnclosedString),
            }
        }

        Ok(Token::String(result))
    }

    fn read_identifier(&mut self) -> Result<Token, LexerError> {
        match self.current {
            Some(c) if is_xid_start(c) => {
                let mut identifier = String::from(c);
                self.advance();

                while let Some(c) = self.current {
                    if is_xid_continue(c) {
                        identifier.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                Ok(Token::from(identifier.as_str()))
            },
            Some(c) => Err(LexerError::UnknownToken(c.to_string())),
            _ => Ok(Token::EOF),
        }
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let mut has_decimal = false;
        let mut number = String::new();

        while let Some(c) = self.current {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Ok(if has_decimal {
            Token::Float(number)
        } else {
            Token::Int(number)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;

    #[test]
    fn str_ascii() {
        let mut lexer = Lexer::new("\"Hello\"");
        let result = lexer.read_string();

        assert_eq!(result.unwrap(), Token::String("Hello".into()));
    }

    #[test]
    fn str_japanese_chars() {
        let mut lexer = Lexer::new("\"こんにちわ\"");
        let result = lexer.read_string();

        assert_eq!(result.unwrap(), Token::String("こんにちわ".into()));
    }

    #[test]
    fn str_emojis() {
        let mut lexer = Lexer::new("\"🦗\"");
        let result = lexer.read_string();

        assert_eq!(result.unwrap(), Token::String("🦗".into()));
    }

    #[test]
    fn ident_ascii() {
        let mut lexer = Lexer::new("Hello");
        let result = lexer.read_identifier();

        assert_eq!(result.unwrap(), Token::Ident("Hello".into()));
    }

    #[test]
    fn ident_japanese_chars() {
        let mut lexer = Lexer::new("こんにちわ");
        let result = lexer.read_identifier();

        assert_eq!(result.unwrap(), Token::Ident("こんにちわ".into()));
    }

    #[test]
    fn ident_underscores() {
        let mut lexer = Lexer::new("hel_lo_");
        let result = lexer.read_identifier();

        assert_eq!(result.unwrap(), Token::Ident("hel_lo_".into()));
    }

    #[test]
    fn number_int_ascii() {
        let mut lexer = Lexer::new("123");
        let result = lexer.read_number();

        assert_eq!(result.unwrap(), Token::Int("123".into()));
    }

    #[test]
    fn number_float_ascii() {
        let mut lexer = Lexer::new("123.123");
        let result = lexer.read_number();

        assert_eq!(result.unwrap(), Token::Float("123.123".into()));
    }
}
