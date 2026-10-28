pub mod cursor;
pub mod error;
pub mod handler;

pub use cursor::*;
pub use error::*;
pub use handler::*;

use crate::tokenizer::{ SpannedToken, Token };

pub struct Grammar<'a, H: GrammarHandler> {
    pub cursor: TokenCursor<'a>,
    pub handler: H,
}

impl<'a, H: GrammarHandler> Grammar<'a, H> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken], handler: H) -> Self {
        Self {
            cursor: TokenCursor::new(source, tokens),
            handler,
        }
    }

    pub fn run(&mut self) -> Result<(), GrammarError> {
        self.block(Token::EOF, Token::EOF)
    }

    fn block(&mut self, open_token: Token, end_token: Token) -> Result<(), GrammarError> {
        self.handler.begin_block(open_token)?;

        if open_token != Token::EOF && self.cursor.current()?.kind == open_token {
            self.handler.consume_block_open(&mut self.cursor)?;
        }

        while
            self.cursor.in_bounds() &&
            (end_token == Token::EOF || self.cursor.current()?.kind != end_token)
        {
            self.handler.skip_entry_trivia(&mut self.cursor)?;

            if
                !self.cursor.in_bounds() ||
                (end_token != Token::EOF && self.cursor.current()?.kind == end_token)
            {
                self.handler.on_before_close(&mut self.cursor)?;
                break;
            }

            let key = match self.cursor.current()?.kind {
                Token::Ident => self.cursor.lexeme()?,
                t => {
                    return Err(
                        (GrammarError::UnexpectedToken {
                            found: t,
                            expected: Token::Ident,
                            line: self.cursor.current()?.line,
                        }).into()
                    );
                }
            };
            let line = self.cursor.current()?.line;

            self.handler.on_key(&mut self.cursor, &key, line)?;
            self.cursor.advance(1);

            self.handler.after_key(&mut self.cursor)?;

            if !self.cursor.in_bounds() {
                return Err(
                    (GrammarError::UnexpectedToken {
                        found: Token::EOF,
                        expected: Token::Equal,
                        line: self.cursor.current()?.line,
                    }).into()
                );
            }

            match self.cursor.current()?.kind {
                Token::Equal => {
                    self.handler.consume_equal(&mut self.cursor)?;
                    self.handler.register_key(&key, line)?;
                    self.expression()?;
                    self.handler.finish_entry(&key, line)?;
                }
                Token::OpenBracket => {
                    self.handler.register_key(&key, line)?;
                    self.handler.enter_struct()?;
                    self.block(Token::OpenBracket, Token::CloseBracket)?;
                    self.handler.exit_struct()?;
                    self.handler.finish_entry(&key, line)?;
                }
                t => {
                    return Err(
                        (GrammarError::UnexpectedToken {
                            found: t,
                            expected: Token::Equal,
                            line: self.cursor.current()?.line,
                        }).into()
                    );
                }
            }

            self.handler.after_entry(&mut self.cursor)?;
        }

        if end_token != Token::EOF {
            if self.cursor.in_bounds() {
                if self.cursor.current()?.kind == end_token {
                    self.handler.consume_block_close(&mut self.cursor)?;
                } else {
                    return Err(
                        (GrammarError::UnterminatedStruct {
                            line: self.cursor.current()?.line,
                        }).into()
                    );
                }
            } else {
                return Err(
                    (GrammarError::UnterminatedStruct {
                        line: self.cursor.current()?.line,
                    }).into()
                );
            }
        }

        self.handler.end_block()?;
        Ok(())
    }

    fn expression(&mut self) -> Result<(), GrammarError> {
        if !self.cursor.in_bounds() {
            return Err(GrammarError::UnexpectedEof.into());
        }

        match self.cursor.current()?.kind {
            Token::String => {
                let lexeme = self.cursor.lexeme()?;
                self.handler.on_string(&lexeme)?;
                self.cursor.advance(1);
                Ok(())
            }
            Token::Ident => {
                let lexeme = self.cursor.lexeme()?;
                self.handler.on_ident(&lexeme)?;
                self.cursor.advance(1);
                Ok(())
            }
            Token::Number => {
                let line = self.cursor.current()?.line;
                let lexeme = self.cursor.lexeme()?;
                self.handler.on_int(&lexeme, line)?;
                self.cursor.advance(1);
                Ok(())
            }
            Token::Float => {
                let line = self.cursor.current()?.line;
                let lexeme = self.cursor.lexeme()?;
                self.handler.on_float(&lexeme, line)?;
                self.cursor.advance(1);
                Ok(())
            }
            Token::OpenSquare => {
                self.handler.consume_array_open(&mut self.cursor)?;

                while self.cursor.in_bounds() && self.cursor.current()?.kind != Token::CloseSquare {
                    self.cursor.skip_whitespace()?;

                    if self.cursor.current()?.kind == Token::CloseSquare {
                        break;
                    }

                    self.expression()?;

                    self.cursor.skip_whitespace()?;

                    if !self.handler.expect_comma(&mut self.cursor)? {
                        self.cursor.skip_whitespace()?;
                        if self.cursor.in_bounds() {
                            match self.cursor.tokens.get(self.cursor.position) {
                                Some(SpannedToken { kind: Token::CloseSquare, .. }) => {
                                    break;
                                }
                                Some(t) => {
                                    return Err(
                                        (GrammarError::UnexpectedToken {
                                            found: t.kind,
                                            expected: Token::Comma,
                                            line: self.cursor.current()?.line,
                                        }).into()
                                    );
                                }
                                None => {
                                    return Err(
                                        (GrammarError::UnterminatedArray {
                                            line: self.cursor.current()?.line,
                                        }).into()
                                    );
                                }
                            }
                        } else {
                            return Err(
                                (GrammarError::UnterminatedArray {
                                    line: self.cursor.current()?.line,
                                }).into()
                            );
                        }
                    }

                    self.handler.after_array_comma(&mut self.cursor)?;
                }

                self.handler.finish_array()?;
                self.handler.consume_array_close(&mut self.cursor)?;
                Ok(())
            }
            t =>
                Err(
                    (GrammarError::ExpectedExpression {
                        found: t,
                        line: self.cursor.current()?.line,
                    }).into()
                ),
        }
    }
}
