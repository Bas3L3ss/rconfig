use crate::tokenizer::{ SpannedToken, Token };

use super::GrammarError;

pub struct TokenCursor<'a> {
    pub source: &'a str,
    pub tokens: &'a [SpannedToken],
    pub position: usize,
}

impl<'a> TokenCursor<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Self {
            source,
            tokens,
            position: 0,
        }
    }

    pub fn advance(&mut self, amount: usize) {
        self.position += amount;
    }

    pub fn current(&self) -> Result<&SpannedToken, GrammarError> {
        match self.tokens.get(self.position) {
            Some(value) => Ok(value),
            None =>
                Err(GrammarError::UnexpectedEof {
                    last: self.tokens.last().expect("unreachable").clone(),
                }),
        }
    }

    pub fn lexeme(&self) -> Result<&'a str, GrammarError> {
        let token = self.current()?;
        Ok(&self.source[token.span.start..token.span.end])
    }

    pub fn expect(&mut self, token: Token) -> Result<bool, GrammarError> {
        if self.in_bounds() && self.current()?.kind == token {
            self.advance(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.position < self.tokens.len() && self.tokens[self.position].kind != Token::EOF
    }

    pub fn skip_trivia(&mut self) -> Result<(), GrammarError> {
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

    pub fn skip_whitespace(&mut self) -> Result<(), GrammarError> {
        while self.in_bounds() {
            match self.current()?.kind {
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

    pub fn skip_just_whitespace(&mut self) -> Result<(), GrammarError> {
        while self.in_bounds() {
            match self.current()?.kind {
                Token::WhiteSpace => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
        Ok(())
    }
}
