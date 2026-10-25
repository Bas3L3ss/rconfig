pub mod value;
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

    pub fn parse(&mut self) -> HashMap<&'a str, Value<'a>> {
        let mut list = HashMap::new();
        while self.in_bounds() {
            self.align_ident();

            if !self.in_bounds() {
                break;
            }

            let key =
                &self.source
                    [self.tokens[self.position].span.start..self.tokens[self.position].span.end];

            self.advance(1);
            self.skip_whitespace();

            if !self.in_bounds() {
                break;
            }

            match &self.tokens[self.position].kind {
                Token::Equal => {
                    self.advance(1);
                    self.skip_whitespace();
                    list.insert(key, self.expression());
                }
                Token::OpenBracket => {
                    self.advance(1);
                    self.skip_whitespace();
                    list.insert(key, self.parse_struct());
                }
                d => {
                    todo!("Incorrect asdasdas {d:?}");
                }
            }
            self.skip_whitespace();
        }
        list
    }

    fn parse_struct(&mut self) -> Value<'a> {
        todo!()
    }

    fn expression(&mut self) -> Value<'a> {
        if !self.in_bounds() {
            todo!("Unexpected EOF");
        }
        match self.current().kind {
            Token::String => {
                let value = Value::String(
                    &self.source[self.current().span.start..self.current().span.end]
                );
                self.advance(1);
                value
            }

            Token::Ident => {
                let ident = &self.source[self.current().span.start..self.current().span.end];

                match ident {
                    "true" => {
                        self.advance(1);
                        Value::Bool(true)
                    }
                    "false" => {
                        self.advance(1);
                        Value::Bool(false)
                    }
                    _ => {
                        let value = Value::String(ident);
                        self.advance(1);
                        value
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

                    // TODO: handle unwrap
                    return Value::Float(float_str.parse::<f64>().unwrap());
                }

                let int_str = &self.source[self.current().span.start..self.current().span.end];
                self.advance(1);
                return Value::Int(int_str.parse::<usize>().unwrap());
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
                        // TODO: handle unwrap
                        return Value::Float(float_str.parse::<f64>().unwrap());
                    }

                    let span_start = self.tokens[self.position].span.start; // minus
                    self.advance(1);
                    let span_end = self.tokens[self.position].span.end; // number
                    let int_str = &self.source[span_start..span_end];
                    self.advance(1);
                    // TODO: handle unwrap
                    return Value::Signed(int_str.parse::<isize>().unwrap());
                } else {
                    todo!(
                        "Expected a number after '-', found {:?}",
                        self.tokens.get(self.position + 1)
                    );
                }
            }

            Token::OpenSquare => {
                let mut vec = vec![];

                self.advance(1);

                while self.in_bounds() && self.current().kind != Token::CloseSquare {
                    self.skip_whitespace();

                    vec.push(self.expression());

                    self.skip_whitespace();

                    self.expect(Token::Comma);
                    self.skip_whitespace();
                }
                self.advance(1);

                return Value::Vec(vec);
            }
            _ => {
                todo!("Incorrect asd");
            }
        }
    }

    fn align_ident(&mut self) {
        while self.in_bounds() {
            match &self.current().kind {
                Token::Ident => {
                    break;
                }
                _ => {
                    self.advance(1);
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
pub fn parse<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> HashMap<&'a str, Value<'a>> {
    Parser::new(source, tokens).parse()
}
