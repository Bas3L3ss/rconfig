pub mod value;
pub mod error;
pub use error::*;
use std::collections::{ HashMap, hash_map::Entry };

pub use value::*;

use crate::tokenizer::{ SpannedToken, Token };

pub type List = HashMap<String, Value>;

pub struct Parser<'a> {
    pub source: &'a str,
    pub tokens: &'a [SpannedToken],
    pub position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Parser {
            source,
            tokens,
            position: 0,
        }
    }

    /// Top-level parser. Parses a block or struct body.
    pub fn parse(&mut self) -> Result<List, ParserError> {
        self.parse_block(Token::EOF, Token::EOF)
    }

    /// Generic function for parsing a block struct (delimited), allowing recursion in parse_struct.
    /// The block ends at the first occurrence of `end_token`, or at EOF if end_token is Token::EOF.
    fn parse_block(&mut self, open_token: Token, end_token: Token) -> Result<List, ParserError> {
        let mut map = HashMap::new();
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
                    return Err(ParserError::UnexpectedToken {
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
                return Err(ParserError::UnexpectedToken {
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
                            entry.insert(line);
                            map.insert(key, self.expression()?);
                        }
                        Entry::Occupied(entry) => {
                            return Err(ParserError::DuplicatedKey {
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
                            map.insert(key, self.parse_struct()?);
                        }
                        Entry::Occupied(entry) => {
                            return Err(ParserError::DuplicatedKey {
                                key,
                                line,
                                first_line: *entry.get(),
                            });
                        }
                    }
                }

                t => {
                    return Err(ParserError::UnexpectedToken {
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
                    return Err(ParserError::UnterminatedStruct {
                        line: self.current()?.line,
                    });
                }
            } else {
                return Err(ParserError::UnterminatedStruct {
                    line: self.current()?.line,
                });
            }
        }

        Ok(map)
    }

    fn parse_struct(&mut self) -> Result<Value, ParserError> {
        Ok(Value::Struct(self.parse_block(Token::OpenBracket, Token::CloseBracket)?))
    }

    fn expression(&mut self) -> Result<Value, ParserError> {
        if !self.in_bounds() {
            return Err(ParserError::UnexpectedEof);
        }
        match self.current()?.kind {
            Token::String => {
                let value = Value::String(
                    self.source[self.current()?.span.start..self.current()?.span.end].to_string()
                );
                self.advance(1);
                Ok(value)
            }
            Token::Ident => {
                let ident =
                    self.source[self.current()?.span.start..self.current()?.span.end].to_string();

                match ident.as_str() {
                    "true" => {
                        self.advance(1);
                        Ok(Value::Bool(true))
                    }
                    "false" => {
                        self.advance(1);
                        Ok(Value::Bool(false))
                    }
                    _ => {
                        let value = Value::String(ident);
                        self.advance(1);
                        Ok(value)
                    }
                }
            }
            Token::Number => {
                let token = self.current()?;
                let line = token.line;
                let lexeme = &self.source[token.span.start..token.span.end];
                self.advance(1);

                if lexeme.contains('.') {
                    Ok(Value::Float(lexeme.parse::<f64>().map_err(|_| {
                        ParserError::InvalidNumber {
                            value: lexeme.to_string(),
                            line,
                        }
                    })?))
                } else {
                    Ok(Value::Int(lexeme.parse::<i64>().map_err(|_| {
                        ParserError::InvalidNumber {
                            value: lexeme.to_string(),
                            line,
                        }
                    })?))
                }
            }

            Token::OpenSquare => {
                let mut vec = vec![];

                self.advance(1);

                while self.in_bounds() && self.current()?.kind != Token::CloseSquare {
                    self.skip_whitespace()?;

                    if self.current()?.kind == Token::CloseSquare {
                        break;
                    }

                    vec.push(self.expression()?);

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
                                    return Err(ParserError::UnexpectedToken {
                                        found: t.kind,
                                        expected: Token::Comma,
                                        line: self.current()?.line,
                                    });
                                }
                                None => {
                                    return Err(ParserError::UnterminatedArray {
                                        line: self.current()?.line,
                                    });
                                }
                            }
                        } else {
                            return Err(ParserError::UnterminatedArray {
                                line: self.current()?.line,
                            });
                        }
                    }
                    self.skip_whitespace()?;
                }
                self.advance(1);

                return Ok(Value::Vec(vec));
            }
            t => {
                return Err(ParserError::ExpectedExpression {
                    found: t,
                    line: self.current()?.line,
                });
            }
        }
    }

    fn skip_trivia(&mut self) -> Result<(), ParserError> {
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

    fn skip_whitespace(&mut self) -> Result<(), ParserError> {
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

    fn current(&self) -> Result<&SpannedToken, ParserError> {
        match &self.tokens.get(self.position) {
            Some(value) => { Ok(value) }
            None => {
                return Err(ParserError::UnexpectedEof);
            }
        }
    }

    fn expect(&mut self, token: Token) -> Result<bool, ParserError> {
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
pub fn parse<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> Result<List, ParserError> {
    Parser::new(source, tokens).parse()
}
