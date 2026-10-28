pub mod error;
pub use error::*;

use crate::tokenizer::{ FormatToken, SpannedToken, Token };

pub type List = Vec<FormatToken>;

pub struct Formatter<'a> {
    source: &'a str,
    tokens: &'a [SpannedToken],
    position: usize,
    formatted_tokens: List,
    identation_level: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Self {
            source,
            tokens,
            position: 0,
            formatted_tokens: Vec::with_capacity(tokens.len()),
            identation_level: 0,
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
        let mut is_first = true;

        if open_token != Token::EOF && self.current()?.kind == open_token {
            self.push_current()?;
            self.push_whitespaces(1);
            self.advance(1);
        }

        while self.in_bounds() && (end_token == Token::EOF || self.current()?.kind != end_token) {
            self.skip_trivia(&mut is_first, open_token)?;

            if !self.in_bounds() || (end_token != Token::EOF && self.current()?.kind == end_token) {
                while let Some(token) = self.formatted_tokens.last() {
                    match token.kind {
                        Token::WhiteSpace | Token::Newline => {
                            self.formatted_tokens.pop();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                break;
            }

            match self.current()?.kind {
                Token::Ident => {
                    self.push_identation(0);
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

            self.skip_whitespace()?;

            self.push_whitespaces(1);

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
                    self.push_whitespaces(1);
                    self.advance(1);
                    self.skip_whitespace()?;
                    self.expression()?;
                }
                Token::OpenBracket => {
                    self.identation_level += 1;
                    self.format_struct()?;
                    self.identation_level -= 1;
                }

                t => {
                    return Err(FormatterError::UnexpectedToken {
                        found: *t,
                        expected: Token::Equal,
                        line: self.current()?.line,
                    });
                }
            }
        }

        if end_token != Token::EOF {
            if self.in_bounds() {
                if self.current()?.kind == end_token {
                    self.push_line();
                    self.push_identation(-1);
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

    fn skip_trivia(
        &mut self,
        is_first: &mut bool,
        open_token: Token
    ) -> Result<(), FormatterError> {
        let mut line = 0;
        while self.in_bounds() {
            match self.current()?.kind {
                Token::Comment => {
                    self.push_current()?;
                    self.advance(1);
                    *is_first = false;
                }

                Token::Newline => {
                    if *is_first {
                        dbg!("hi");
                        if open_token == Token::EOF {
                            self.advance(1);
                            continue;
                        } else if line < 2 {
                            line = 2;
                            self.push_current()?;
                            self.advance(1);
                            continue;
                        }
                    }
                    if line < 2 {
                        self.push_current()?;
                        line += 1;
                    }
                    self.advance(1);
                }

                Token::WhiteSpace => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
        if *is_first && open_token != Token::EOF && line == 0 {
            self.push_line();
        }
        *is_first = false;
        Ok(())
    }

    fn skip_whitespace(&mut self) -> Result<(), FormatterError> {
        while self.in_bounds() {
            match &self.current()?.kind {
                Token::WhiteSpace | Token::Newline => {
                    // self.push_current()?;
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

    fn push_identation(&mut self, regress: isize) {
        self.formatted_tokens.push(
            FormatToken::new(
                Token::WhiteSpace,
                "\t".repeat(((self.identation_level as isize) + regress) as usize)
            )
        );
    }

    fn push_line(&mut self) {
        self.formatted_tokens.push(FormatToken::new(Token::WhiteSpace, "\n".to_string()));
    }

    fn push_whitespaces(&mut self, amount: usize) {
        self.formatted_tokens.push(FormatToken::new(Token::WhiteSpace, " ".repeat(amount)));
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
