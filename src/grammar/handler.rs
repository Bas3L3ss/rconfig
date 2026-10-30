use crate::tokenizer::{ SpannedToken, Token };

use super::{ GrammarError, TokenCursor };

/// Semantic hooks for the shared grammar walk.
///
/// Defaults match parser/validator behavior. Formatter overrides the hooks that
/// emit tokens or customize trivia.
pub trait GrammarHandler {
    fn skip_entry_trivia(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_trivia().map_err(Into::into)
    }

    /// Current token is the block opener (`{`). Consume it (and any post-open trivia).
    fn consume_block_open(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.advance(1);
        Ok(())
    }

    /// About to leave the block loop because the end delimiter was reached.
    fn on_before_close(&mut self, _cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        Ok(())
    }

    /// Current token is the block closer (`}`). Consume it.
    fn consume_block_close(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.advance(1);
        Ok(())
    }

    /// Current token is the entry key. Side effects only; driver advances afterward.
    fn on_key(
        &mut self,
        _cursor: &mut TokenCursor<'_>,
        _key: &str,
        _line: usize
    ) -> Result<(), GrammarError> {
        Ok(())
    }

    /// After the key token has been consumed.
    fn after_key(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_whitespace().map_err(Into::into)
    }

    /// Register a key before parsing its value. Parser/validator check duplicates.
    fn register_key(&mut self, _key: &str, _token: &SpannedToken) -> Result<(), GrammarError> {
        Ok(())
    }

    /// Current token is `=`. Consume it and trailing whitespace.
    fn consume_equal(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.advance(1);
        cursor.skip_whitespace().map_err(Into::into)
    }

    fn enter_struct(&mut self) -> Result<(), GrammarError> {
        Ok(())
    }

    fn exit_struct(&mut self) -> Result<(), GrammarError> {
        Ok(())
    }

    /// Called after a completed `key = expr` or `key { ... }` entry's value is ready.
    fn finish_entry(&mut self, _key: &str, _line: usize) -> Result<(), GrammarError> {
        Ok(())
    }

    /// After an entry is fully processed.
    fn after_entry(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_whitespace().map_err(Into::into)
    }

    fn on_string(&mut self, _lexeme: &str) -> Result<(), GrammarError> {
        Ok(())
    }

    fn on_ident(&mut self, _lexeme: &str) -> Result<(), GrammarError> {
        Ok(())
    }

    fn on_int(&mut self, lexeme: &str, token: &SpannedToken) -> Result<(), GrammarError> {
        lexeme.parse::<i64>().map_err(|_| {
            GrammarError::InvalidNumber {
                value: lexeme.to_string(),
                token: token.clone(),
            }
        })?;
        Ok(())
    }

    fn on_float(&mut self, lexeme: &str, token: &SpannedToken) -> Result<(), GrammarError> {
        lexeme.parse::<f64>().map_err(|_| {
            GrammarError::InvalidNumber {
                value: lexeme.to_string(),
                token: token.clone(),
            }
        })?;
        Ok(())
    }

    /// Current token is `[`. Consume it.
    fn consume_array_open(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.advance(1);
        Ok(())
    }

    /// Called when a comma was successfully consumed after an array element.
    fn after_array_comma(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_whitespace().map_err(Into::into)
    }

    /// Try to consume a comma. Returns whether one was present.
    fn expect_comma(&mut self, cursor: &mut TokenCursor<'_>) -> Result<bool, GrammarError> {
        cursor.expect(Token::Comma).map_err(Into::into)
    }

    /// Current token is `]` (or about to close). Consume the closer.
    fn consume_array_close(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.advance(1);
        Ok(())
    }

    /// Called after all array elements are parsed, before consuming `]`.
    fn finish_array(&mut self) -> Result<(), GrammarError> {
        Ok(())
    }

    fn begin_block(&mut self, _open_token: Token) -> Result<(), GrammarError> {
        Ok(())
    }

    fn end_block(&mut self) -> Result<(), GrammarError> {
        Ok(())
    }
}
