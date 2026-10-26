pub mod value;
pub mod error;
pub use error::*;
use std::collections::HashMap;

pub use value::*;

use crate::tokenizer::{ SpannedToken, Token };

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

    /// Helper to check if current position is within tokens and not EOF
    fn in_bounds(&self) -> bool {
        self.position < self.tokens.len() && self.tokens[self.position].kind != Token::EOF
    }

    /// Top-level parser. Parses a block or struct body.
    pub fn parse(&mut self) -> Result<HashMap<String, Value>, ParserError> {
        self.parse_block(Token::EOF, Token::EOF)
    }

    /// Generic function for parsing a block struct (delimited), allowing recursion in parse_struct.
    /// The block ends at the first occurrence of `end_token`, or at EOF if end_token is Token::EOF.
    fn parse_block(
        &mut self,
        open_token: Token,
        end_token: Token
    ) -> Result<HashMap<String, Value>, ParserError> {
        let mut map = HashMap::new();

        if open_token != Token::EOF && self.current().kind == open_token {
            self.advance(1);
        }

        while self.in_bounds() && (end_token == Token::EOF || self.current().kind != end_token) {
            self.skip_trivia();

            if !self.in_bounds() || (end_token != Token::EOF && self.current().kind == end_token) {
                break;
            }

            let key =
                self.source[
                    self.tokens[self.position].span.start..self.tokens[self.position].span.end
                ].to_string();
            self.advance(1);
            self.skip_whitespace();

            if !self.in_bounds() {
                break;
            }

            match &self.current().kind {
                Token::Equal => {
                    self.advance(1);
                    self.skip_whitespace();
                    map.insert(key, self.expression()?);
                }
                Token::OpenBracket => {
                    map.insert(key, self.parse_struct()?);
                }
                Token::Comment => {
                    self.advance(1);
                    self.skip_whitespace();
                }
                t => {
                    return Err(ParserError::UnexpectedToken {
                        found: *t,
                        expected: Token::Equal,
                        line: Line(self.current().line),
                    });
                }
            }
            self.skip_whitespace();
        }

        if end_token != Token::EOF && self.in_bounds() && self.current().kind == end_token {
            self.advance(1);
        }

        Ok(map)
    }

    fn parse_struct(&mut self) -> Result<Value, ParserError> {
        // Assume the { has just been seen (current() is OpenBracket).
        // Delegate to parse_block, which handles recursively.
        // parse_block will automatically advance past the OpenBracket and handle closing bracket.
        Ok(Value::Struct(self.parse_block(Token::OpenBracket, Token::CloseBracket)?))
    }

    fn expression(&mut self) -> Result<Value, ParserError> {
        if !self.in_bounds() {
            return Err(ParserError::UnexpectedEof);
        }
        match self.current().kind {
            Token::String => {
                let value = Value::String(
                    self.source[self.current().span.start..self.current().span.end].to_string()
                );
                self.advance(1);
                Ok(value)
            }
            Token::Ident => {
                let ident =
                    self.source[self.current().span.start..self.current().span.end].to_string();

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
                if
                    self.position + 2 < self.tokens.len() &&
                    self.tokens[self.position + 1].kind == Token::Dot &&
                    self.tokens[self.position + 2].kind == Token::Number
                {
                    let span_start = self.current().span.start;
                    let span_end = self.tokens[self.position + 2].span.end;
                    let float_str = &self.source[span_start..span_end];
                    self.advance(3);

                    return Ok(
                        Value::Float(
                            float_str
                                .parse::<f64>()
                                .map_err(|_|
                                    ParserError::InvalidNumber(
                                        float_str.to_string(),
                                        Line(self.current().line)
                                    )
                                )?
                        )
                    );
                }

                let int_str = &self.source[self.current().span.start..self.current().span.end];
                self.advance(1);

                return Ok(
                    Value::Int(
                        int_str
                            .parse::<usize>()
                            .map_err(|_|
                                ParserError::InvalidNumber(
                                    int_str.to_string(),
                                    Line(self.current().line)
                                )
                            )?
                    )
                );
            }
            Token::Minus => {
                if
                    self.position + 1 < self.tokens.len() &&
                    self.tokens[self.position + 1].kind == Token::Number
                {
                    if
                        self.position + 3 < self.tokens.len() &&
                        self.tokens[self.position + 2].kind == Token::Dot &&
                        self.tokens[self.position + 3].kind == Token::Number
                    {
                        let span_start = self.tokens[self.position].span.start;
                        let span_end = self.tokens[self.position + 3].span.end;
                        let float_str = &self.source[span_start..span_end];
                        self.advance(4);

                        return Ok(
                            Value::Float(
                                float_str
                                    .parse::<f64>()
                                    .map_err(|_|
                                        ParserError::InvalidNumber(
                                            float_str.to_string(),
                                            Line(self.current().line)
                                        )
                                    )?
                            )
                        );
                    }

                    let span_start = self.tokens[self.position].span.start; // minus
                    self.advance(1);
                    let span_end = self.tokens[self.position].span.end; // number
                    let int_str = &self.source[span_start..span_end];
                    self.advance(1);
                    return Ok(
                        Value::Signed(
                            int_str
                                .parse::<isize>()
                                .map_err(|_|
                                    ParserError::InvalidNumber(
                                        int_str.to_string(),
                                        Line(self.current().line)
                                    )
                                )?
                        )
                    );
                } else {
                    return Err(
                        ParserError::InvalidSigned(
                            self.tokens
                                .get(self.position + 1)
                                .map(|token| format!("{:?}", token.kind))
                                .unwrap_or_else(|| "nothing".to_string()),
                            Line(self.current().line)
                        )
                    );
                }
            }

            Token::OpenSquare => {
                let mut vec = vec![];

                self.advance(1);

                while self.in_bounds() && self.current().kind != Token::CloseSquare {
                    self.skip_whitespace();

                    vec.push(self.expression()?);

                    self.skip_whitespace();

                    if !self.expect(Token::Comma) {
                        self.skip_whitespace();
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
                                        line: Line(self.current().line),
                                    });
                                }
                                None => {
                                    return Err(
                                        ParserError::UnterminatedArray(Line(self.current().line))
                                    );
                                }
                            }
                        } else {
                            return Err(ParserError::UnterminatedArray(Line(self.current().line)));
                        }
                    }
                    self.skip_whitespace();
                }
                self.advance(1);

                return Ok(Value::Vec(vec));
            }
            t => {
                return Err(ParserError::ExpectedExpression(t, Line(self.current().line)));
            }
        }
    }

    fn skip_trivia(&mut self) {
        while self.in_bounds() {
            match self.current().kind {
                Token::WhiteSpace | Token::Newline | Token::Comment => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.in_bounds() {
            match &self.current().kind {
                Token::WhiteSpace | Token::Newline => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn advance(&mut self, amount: usize) {
        self.position += amount;
    }

    fn current(&self) -> &SpannedToken {
        &self.tokens[self.position]
    }

    fn expect(&mut self, token: Token) -> bool {
        if self.in_bounds() && self.current().kind == token {
            self.advance(1);
            true
        } else {
            false
        }
    }
}

// Add a free function for old interface compatibility, if desired
pub fn parse<'a>(
    source: &'a str,
    tokens: &'a [SpannedToken]
) -> Result<HashMap<String, Value>, ParserError> {
    Parser::new(source, tokens).parse()
}
