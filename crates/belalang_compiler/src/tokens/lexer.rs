use std::iter::Peekable;
use std::str::Chars;

use crate::error::SyntaxError;
use crate::tokens::Token;

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

        Lexer {
            current,
            chars,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.current;
        self.current = self.chars.next();
        result
    }

    pub fn next_token(&mut self) -> Result<Token, SyntaxError> {
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
                }
                // skips all empty whitespaces
                Some(' ' | '\t' | '\n' | '\r') => {
                    self.advance();
                }
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
                    }
                    _ => Err(SyntaxError::UnknownToken(":".into())),
                }
            },
            Some('=') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Eq)
                    }
                    _ => Ok(Token::Assign),
                }
            },
            Some('!') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Ne)
                    }
                    _ => Ok(Token::Not),
                }
            },
            Some('&') => {
                self.advance();
                match self.current {
                    Some('&') => {
                        self.advance();
                        Ok(Token::And)
                    }
                    Some('=') => {
                        self.advance();
                        Ok(Token::BitAndAssign)
                    }
                    _ => Ok(Token::BitAnd),
                }
            },
            Some('|') => {
                self.advance();
                match self.current {
                    Some('|') => {
                        self.advance();
                        Ok(Token::Or)
                    }
                    Some('=') => {
                        self.advance();
                        Ok(Token::BitOrAssign)
                    }
                    _ => Ok(Token::BitOr),
                }
            },
            Some('^') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::BitXorAssign)
                    }
                    _ => Ok(Token::BitXor),
                }
            },
            Some('<') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Le)
                    }
                    Some('<') => {
                        self.advance();
                        match self.chars.peek() {
                            Some('=') => {
                                self.advance();
                                Ok(Token::ShiftLeftAssign)
                            }
                            _ => Ok(Token::ShiftLeft),
                        }
                    }
                    _ => Ok(Token::Lt),
                }
            },
            Some('>') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::Ge)
                    }
                    Some('>') => {
                        self.advance();
                        match self.chars.peek() {
                            Some('=') => {
                                self.advance();
                                Ok(Token::ShiftRightAssign)
                            }
                            _ => Ok(Token::ShiftRight),
                        }
                    }
                    _ => Ok(Token::Gt),
                }
            },
            Some('+') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::AddAssign)
                    }
                    _ => Ok(Token::Add),
                }
            },
            Some('-') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::SubAssign)
                    }
                    _ => Ok(Token::Sub),
                }
            },
            Some('*') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::MulAssign)
                    }
                    _ => Ok(Token::Mul),
                }
            },
            Some('/') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::DivAssign)
                    }
                    _ => Ok(Token::Div),
                }
            },
            Some('%') => {
                self.advance();
                match self.current {
                    Some('=') => {
                        self.advance();
                        Ok(Token::ModAssign)
                    }
                    _ => Ok(Token::Mod),
                }
            },
            Some('(') => { self.advance(); Ok(Token::LeftParen) },
            Some(')') => { self.advance(); Ok(Token::RightParen) },
            Some('{') => { self.advance(); Ok(Token::LeftBrace) },
            Some('}') => { self.advance(); Ok(Token::RightBrace) },
            Some('[') => { self.advance(); Ok(Token::LeftBracket) },
            Some(']') => { self.advance(); Ok(Token::RightBracket) },
            Some(';') => { self.advance(); Ok(Token::Semicolon) },
            Some(',') => { self.advance(); Ok(Token::Comma) },
            Some('\\') => { self.advance(); Ok(Token::Backslash) },
            Some('"') => self.read_string(),
            Some(c) if c.is_alphabetic() => Ok(self.read_identifier()?),
            Some(c) if c.is_numeric() => Ok(self.read_number()?),
            Some(c) => Err(SyntaxError::UnknownToken(c.to_string())),
            _ => unreachable!(),
        }
    }

    pub fn read_string(&mut self) -> Result<Token, SyntaxError> {
        self.advance(); // consume the opening "
        let mut result = String::new();

        loop {
            match self.advance() {
                Some('\\') => match self.current {
                    Some('n') => {
                        self.advance();
                        result.push('\n');
                    }
                    Some('r') => {
                        self.advance();
                        result.push('\r');
                    }
                    Some('t') => {
                        self.advance();
                        result.push('\t');
                    }
                    Some('"') => {
                        self.advance();
                        result.push('"');
                    }
                    Some('\\') => {
                        self.advance();
                        result.push('\\');
                    }
                    Some('x') => {
                        self.advance(); // consume the 'x'

                        match (
                            self.advance().and_then(char_to_u8),
                            self.advance().and_then(char_to_u8),
                        ) {
                            (Some(hi), Some(lo)) => result.push(((hi << 4) | lo) as char),
                            (_, _) => return Err(SyntaxError::UnknownEscapeString),
                        }
                    }
                    Some(_) => return Err(SyntaxError::UnknownEscapeString),
                    None => return Err(SyntaxError::UnclosedString()),
                },
                Some('"') => break,
                Some(c) => result.push(c),
                None => return Err(SyntaxError::UnclosedString()),
            }
        }

        Ok(Token::String(result))
    }

    pub fn read_identifier(&mut self) -> Result<Token, SyntaxError> {
        let mut identifier = String::new();

        while let Some(c) = self.current {
            if c.is_alphanumeric() {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::try_from(identifier.as_str())
    }

    pub fn read_number(&mut self) -> Result<Token, SyntaxError> {
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
    use super::Token;
    use super::Lexer;

    #[test]
    fn test_valid_utf8() {
        let mut lexer = Lexer::new("\"Hello\"");
        let result = lexer.read_string();
        
        assert_eq!(result.unwrap(), Token::String("Hello".into()));
    }

    #[test]
    fn test_japanese_chars() {
        let mut lexer = Lexer::new("\"こんにちわ\"");
        let result = lexer.read_string();

        assert_eq!(result.unwrap(), Token::String("こんにちわ".into()));
    }

    #[test]
    fn test_emojis() {
        let mut lexer = Lexer::new("\"🦗\"");
        let result = lexer.read_string();

        assert_eq!(result.unwrap(), Token::String("🦗".into()));
    }

    #[test]
    fn test_valid_utf8_ident() {
        let mut lexer = Lexer::new("Hello");
        let result = lexer.read_identifier();

        assert_eq!(result.unwrap(), Token::Ident("Hello".into()));
    }

    #[test]
    fn test_japanese_ident() {
        let mut lexer = Lexer::new("こんにちわ");
        let result = lexer.read_identifier();

        assert_eq!(result.unwrap(), Token::Ident("こんにちわ".into()));
    }
}
