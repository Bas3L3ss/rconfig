pub mod error;
pub use error::*;
use std::collections::{ HashMap, hash_map::Entry };

use crate::tokenizer::{ SpannedToken, Token };

pub struct Validator<'a> {
    source: &'a str,
    tokens: &'a [SpannedToken],
    position: usize,
}

impl<'a> Validator<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Self {
            source,
            tokens,
            position: 0,
        }
    }

    pub fn validate(&mut self) -> Result<(), ValidatorError> {
        self.validate_block(Token::EOF, Token::EOF)
    }

    fn validate_block(
        &mut self,
        open_token: Token,
        end_token: Token
    ) -> Result<(), ValidatorError> {
        let mut linemap = HashMap::new();

        if open_token != Token::EOF && self.current()?.kind == open_token {
            self.advance(1);
        }

        while self.in_bounds() && (end_token == Token::EOF || self.current()?.kind != end_token) {
            self.skip_trivia()?;

            if !self.in_bounds() || (end_token != Token::EOF && self.current()?.kind == end_token) {
                break;
            }

            let key = match self.current()?.kind {
                Token::Ident => {
                    self.source[self.current()?.span.start..self.current()?.span.end].to_string()
                }
                t => {
                    return Err(ValidatorError::UnexpectedToken {
                        found: t,
                        expected: Token::Ident,
                        line: self.current()?.line,
                    });
                }
            };
            let line = self.current()?.line;

            self.advance(1);

            self.skip_whitespace()?;

            if !self.in_bounds() {
                return Err(ValidatorError::UnexpectedToken {
                    found: Token::EOF,
                    expected: Token::Equal,
                    line: self.current()?.line,
                });
            }

            match &self.current()?.kind {
                Token::Equal => {
                    self.advance(1);
                    self.skip_whitespace()?;

                    match linemap.entry(key.clone()) {
                        Entry::Vacant(entry) => {
                            entry.insert(line); // WIP: parser, validator
                            self.expression()?;
                        }
                        Entry::Occupied(entry) => {
                            return Err(ValidatorError::DuplicatedKey {
                                key,
                                line,
                                first_line: *entry.get(),
                            });
                        }
                    }
                }
                Token::OpenBracket => {
                    match linemap.entry(key.clone()) {
                        Entry::Vacant(entry) => {
                            entry.insert(line);
                            self.validate_struct()?;
                        }
                        Entry::Occupied(entry) => {
                            return Err(ValidatorError::DuplicatedKey {
                                key,
                                line,
                                first_line: *entry.get(),
                            });
                        }
                    }
                }

                t => {
                    return Err(ValidatorError::UnexpectedToken {
                        found: *t,
                        expected: Token::Equal,
                        line: self.current()?.line,
                    });
                }
            }

            self.skip_whitespace()?;
        }

        if end_token != Token::EOF {
            if self.in_bounds() {
                if self.current()?.kind == end_token {
                    self.advance(1);
                } else {
                    return Err(ValidatorError::UnterminatedStruct {
                        line: self.current()?.line,
                    });
                }
            } else {
                return Err(ValidatorError::UnterminatedStruct {
                    line: self.current()?.line,
                });
            }
        }

        Ok(())
    }

    fn validate_struct(&mut self) -> Result<(), ValidatorError> {
        self.validate_block(Token::OpenBracket, Token::CloseBracket)?;
        Ok(())
    }

    fn expression(&mut self) -> Result<(), ValidatorError> {
        if !self.in_bounds() {
            return Err(ValidatorError::UnexpectedEof);
        }
        match self.current()?.kind {
            Token::String | Token::Ident => {
                self.advance(1);
                Ok(())
            }
            Token::Number => {
                let token = self.current()?;
                let line = token.line;
                let lexeme = &self.source[token.span.start..token.span.end];
                lexeme.parse::<i64>().map_err(|_| {
                    ValidatorError::InvalidNumber {
                        value: lexeme.to_string(),
                        line,
                    }
                })?;
                self.advance(1);
                Ok(())
            }

            Token::Float => {
                let token = self.current()?;
                let line = token.line;
                let lexeme = &self.source[token.span.start..token.span.end];
                self.advance(1);
                lexeme.parse::<f64>().map_err(|_| {
                    ValidatorError::InvalidNumber {
                        value: lexeme.to_string(),
                        line,
                    }
                })?;

                Ok(())
            }

            Token::OpenSquare => {
                self.advance(1);

                while self.in_bounds() && self.current()?.kind != Token::CloseSquare {
                    self.skip_whitespace()?;

                    if self.current()?.kind == Token::CloseSquare {
                        break;
                    }

                    self.expression()?;

                    self.skip_whitespace()?;

                    if !self.expect(Token::Comma)? {
                        self.skip_whitespace()?;
                        if self.in_bounds() {
                            match self.tokens.get(self.position) {
                                Some(
                                    SpannedToken { kind: Token::CloseSquare, span: _, line: _ },
                                ) => {
                                    break;
                                }
                                Some(t) => {
                                    return Err(ValidatorError::UnexpectedToken {
                                        found: t.kind,
                                        expected: Token::Comma,
                                        line: self.current()?.line,
                                    });
                                }
                                None => {
                                    return Err(ValidatorError::UnterminatedArray {
                                        line: self.current()?.line,
                                    });
                                }
                            }
                        } else {
                            return Err(ValidatorError::UnterminatedArray {
                                line: self.current()?.line,
                            });
                        }
                    }
                    self.skip_whitespace()?;
                }
                self.advance(1);

                return Ok(());
            }
            t => {
                return Err(ValidatorError::ExpectedExpression {
                    found: t,
                    line: self.current()?.line,
                });
            }
        }
    }

    fn skip_trivia(&mut self) -> Result<(), ValidatorError> {
        while self.in_bounds() {
            match self.current()?.kind {
                Token::WhiteSpace | Token::Newline | Token::Comment => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) -> Result<(), ValidatorError> {
        while self.in_bounds() {
            match &self.current()?.kind {
                Token::WhiteSpace | Token::Newline => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
        Ok(())
    }

    fn advance(&mut self, amount: usize) {
        self.position += amount;
    }

    fn current(&self) -> Result<&SpannedToken, ValidatorError> {
        match &self.tokens.get(self.position) {
            Some(value) => { Ok(value) }
            None => {
                return Err(ValidatorError::UnexpectedEof);
            }
        }
    }

    fn expect(&mut self, token: Token) -> Result<bool, ValidatorError> {
        if self.in_bounds() && self.current()?.kind == token {
            self.advance(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Helper to check if current position is within tokens and not EOF
    fn in_bounds(&self) -> bool {
        self.position < self.tokens.len() && self.tokens[self.position].kind != Token::EOF
    }
}

// Add a free function for old interface compatibility, if desired
pub fn validate<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> Result<(), ValidatorError> {
    Validator::new(source, tokens).validate()
}
