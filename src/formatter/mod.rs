pub mod error;
pub use error::*;

use crate::tokenizer::{ FormatToken, SpannedToken, Token };

pub type List = Vec<FormatToken>;

pub struct Formatter<'a> {
    pub source: &'a str,
    pub tokens: &'a [SpannedToken],
    pub position: usize,
    pub formatted_tokens: List,
}

impl<'a> Formatter<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Self {
            source,
            tokens,
            position: 0,
            formatted_tokens: Vec::with_capacity(tokens.len()),
        }
    }

    /// Top-level parser. Parses a block or struct body.
    pub fn format(mut self) -> Result<List, FormatterError> {
        self.format_block(Token::EOF, Token::EOF)?;
        Ok(self.formatted_tokens)
    }

    /// Generic function for parsing a block struct (delimited), allowing recursion in parse_struct.
    /// The block ends at the first occurrence of `end_token`, or at EOF if end_token is Token::EOF.
    fn format_block(&mut self, open_token: Token, end_token: Token) -> Result<(), FormatterError> {
        if open_token != Token::EOF && self.current()?.kind == open_token {
            self.formatted_tokens.push(FormatToken::new(self.current()?.kind, String::from("{")));
            self.advance(1);
        }

        while self.in_bounds() && (end_token == Token::EOF || self.current()?.kind != end_token) {
            // WIP2: THIS WILL BE A HANDLER
            self.skip_trivia()?;

            if !self.in_bounds() || (end_token != Token::EOF && self.current()?.kind == end_token) {
                break;
            }

            match self.current()?.kind {
                Token::Ident => {
                    self.push_current()?;
                }
                t => {
                    return Err(FormatterError::UnexpectedToken {
                        found: t,
                        expected: Token::Ident,
                        line: self.current()?.line,
                    });
                }
            }

            self.advance(1);

            // WIP2: THIS WILL BE A HANDLER
            self.skip_whitespace()?;

            if !self.in_bounds() {
                return Err(FormatterError::UnexpectedToken {
                    found: Token::EOF,
                    expected: Token::Equal,
                    line: self.current()?.line,
                });
            }

            match &self.current()?.kind {
                Token::Equal => {
                    self.push_current()?;
                    self.advance(1);
                    self.skip_whitespace()?;

                    self.expression()?;
                }
                Token::OpenBracket => {
                    self.format_struct()?;
                }

                t => {
                    return Err(FormatterError::UnexpectedToken {
                        found: *t,
                        expected: Token::Equal,
                        line: self.current()?.line,
                    });
                }
            }

            // WIP2: THIS WILL BE A HANDLER
            self.skip_whitespace()?;
        }

        if end_token != Token::EOF {
            if self.in_bounds() {
                if self.current()?.kind == end_token {
                    // WIP2: THIS WILL BE A HANDLER
                    self.push_current()?;
                    self.advance(1);
                } else {
                    return Err(FormatterError::UnterminatedStruct {
                        line: self.current()?.line,
                    });
                }
            } else {
                return Err(FormatterError::UnterminatedStruct {
                    line: self.current()?.line,
                });
            }
        }

        Ok(())
    }

    fn format_struct(&mut self) -> Result<(), FormatterError> {
        Ok(self.format_block(Token::OpenBracket, Token::CloseBracket)?)
    }

    fn expression(&mut self) -> Result<(), FormatterError> {
        if !self.in_bounds() {
            return Err(FormatterError::UnexpectedEof);
        }
        match self.current()?.kind {
            Token::String => {
                let lexeme =
                    self.source[self.current()?.span.start..self.current()?.span.end].to_string();
                self.formatted_tokens.push(FormatToken::new(Token::String, lexeme));
                self.advance(1);
                Ok(())
            }

            Token::Ident => {
                // WIP2: THIS WILL BE A HANDLER
                let ident =
                    self.source[self.current()?.span.start..self.current()?.span.end].to_string();

                match ident.as_str() {
                    "true" => {
                        self.formatted_tokens.push(FormatToken::new(Token::Ident, ident));
                        self.advance(1);
                        Ok(())
                    }
                    "false" => {
                        self.formatted_tokens.push(FormatToken::new(Token::Ident, ident));
                        self.advance(1);
                        Ok(())
                    }
                    _ => {
                        self.formatted_tokens.push(FormatToken::new(Token::String, ident));
                        self.advance(1);
                        Ok(())
                    }
                }
            }

            Token::Number => {
                let token = self.current()?;
                let line = token.line;
                let lexeme = &self.source[token.span.start..token.span.end];

                self.formatted_tokens.push(
                    FormatToken::new(
                        Token::Number,
                        lexeme
                            .parse::<i64>()
                            .map_err(|_| {
                                FormatterError::InvalidNumber {
                                    value: lexeme.to_string(),
                                    line,
                                }
                            })?
                            .to_string()
                    )
                );
                self.advance(1);

                Ok(())
            }

            Token::Float => {
                let token = self.current()?;
                let line = token.line;
                let lexeme = &self.source[token.span.start..token.span.end];

                self.formatted_tokens.push(
                    FormatToken::new(
                        Token::Number,
                        lexeme
                            .parse::<f64>()
                            .map_err(|_| {
                                FormatterError::InvalidNumber {
                                    value: lexeme.to_string(),
                                    line,
                                }
                            })?
                            .to_string()
                    )
                );

                self.advance(1);

                Ok(())
            }

            Token::OpenSquare => {
                self.push_current()?;

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
                                    return Err(FormatterError::UnexpectedToken {
                                        found: t.kind,
                                        expected: Token::Comma,
                                        line: self.current()?.line,
                                    });
                                }
                                None => {
                                    return Err(FormatterError::UnterminatedArray {
                                        line: self.current()?.line,
                                    });
                                }
                            }
                        } else {
                            return Err(FormatterError::UnterminatedArray {
                                line: self.current()?.line,
                            });
                        }
                    }

                    self.skip_whitespace()?;
                }
                self.push_current()?;
                self.advance(1);

                return Ok(());
            }
            t => {
                return Err(FormatterError::ExpectedExpression {
                    found: t,
                    line: self.current()?.line,
                });
            }
        }
    }

    fn skip_trivia(&mut self) -> Result<(), FormatterError> {
        while self.in_bounds() {
            match self.current()?.kind {
                Token::WhiteSpace | Token::Comment | Token::Newline => {
                    self.push_current()?;
                    self.advance(1);
                }

                _ => {
                    break;
                }
            }
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) -> Result<(), FormatterError> {
        while self.in_bounds() {
            match &self.current()?.kind {
                Token::WhiteSpace | Token::Newline => {
                    self.push_current()?;
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

    fn current(&self) -> Result<&SpannedToken, FormatterError> {
        match &self.tokens.get(self.position) {
            Some(value) => { Ok(value) }
            None => {
                return Err(FormatterError::UnexpectedEof);
            }
        }
    }

    fn expect(&mut self, token: Token) -> Result<bool, FormatterError> {
        if self.in_bounds() && self.current()?.kind == token {
            self.push_current()?;

            self.advance(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn push_current(&mut self) -> Result<(), FormatterError> {
        let lexeme = self.source[self.current()?.span.start..self.current()?.span.end].to_string();
        self.formatted_tokens.push(FormatToken::new(self.current()?.kind, lexeme));
        Ok(())
    }

    /// Helper to check if current position is within tokens and not EOF
    fn in_bounds(&self) -> bool {
        self.position < self.tokens.len() && self.tokens[self.position].kind != Token::EOF
    }
}

// Add a free function for old interface compatibility, if desired
pub fn format<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> Result<List, FormatterError> {
    Formatter::new(source, tokens).format()
}
