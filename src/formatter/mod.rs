use crate::grammar::{ Grammar, GrammarError, GrammarHandler, TokenCursor };
use crate::tokenizer::{ FormatToken, SpannedToken, Token };

pub type List = Vec<FormatToken>;

struct BlockFmtState {
    is_first: bool,
    open_token: Token,
}

struct FormatterHandler {
    source: String,
    formatted_tokens: List,
    identation_level: usize,
    blocks: Vec<BlockFmtState>,
}

impl FormatterHandler {
    fn new(source: &str, token_capacity: usize) -> Self {
        Self {
            source: source.to_string(),
            formatted_tokens: Vec::with_capacity(token_capacity),
            identation_level: 0,
            blocks: Vec::new(),
        }
    }

    fn into_list(self) -> List {
        self.formatted_tokens
    }

    fn push_current(&mut self, cursor: &TokenCursor<'_>) -> Result<(), GrammarError> {
        let token = cursor.current()?;
        let lexeme = self.source[token.span.start..token.span.end].to_string();
        self.formatted_tokens.push(FormatToken::new(token.kind, lexeme));
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
}

impl GrammarHandler for FormatterHandler {
    fn begin_block(&mut self, open_token: Token) -> Result<(), GrammarError> {
        self.blocks.push(BlockFmtState {
            is_first: true,
            open_token,
        });
        Ok(())
    }

    fn end_block(&mut self) -> Result<(), GrammarError> {
        self.blocks.pop();
        Ok(())
    }

    fn skip_entry_trivia(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        let block = self.blocks.last_mut().expect("active format block");
        let open_token = block.open_token;
        let mut is_first = block.is_first;
        let mut line = 0;

        while cursor.in_bounds() {
            match cursor.current()?.kind {
                Token::Comment => {
                    self.push_identation(0);
                    let token = cursor.current()?;
                    let lexeme = self.source[token.span.start..token.span.end].to_string();
                    self.formatted_tokens.push(FormatToken::new(token.kind, lexeme));
                    cursor.advance(1);
                    is_first = false;
                    line = 0;
                }
                Token::Newline => {
                    if is_first {
                        if open_token == Token::EOF {
                            cursor.advance(1);
                            continue;
                        } else if line < 2 {
                            line = 2;
                            let token = cursor.current()?;
                            let lexeme = self.source[token.span.start..token.span.end].to_string();
                            self.formatted_tokens.push(FormatToken::new(token.kind, lexeme));
                            cursor.advance(1);
                            continue;
                        }
                    }
                    if line < 2 {
                        let token = cursor.current()?;
                        let lexeme = self.source[token.span.start..token.span.end].to_string();
                        self.formatted_tokens.push(FormatToken::new(token.kind, lexeme));
                        line += 1;
                    }
                    cursor.advance(1);
                }
                Token::WhiteSpace => {
                    cursor.advance(1);
                }
                _ => {
                    break;
                }
            }
        }

        if is_first && open_token != Token::EOF && line == 0 {
            self.formatted_tokens.push(FormatToken::new(Token::WhiteSpace, "\n".to_string()));
        }

        let block = self.blocks.last_mut().expect("active format block");
        block.is_first = false;
        Ok(())
    }

    fn consume_block_open(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        self.push_current(cursor)?;
        cursor.advance(1);
        cursor.skip_just_whitespace()?;
        Ok(())
    }

    fn on_before_close(&mut self, _cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
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
        Ok(())
    }

    fn consume_block_close(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        self.push_line();
        self.push_identation(-1);
        self.push_current(cursor)?;
        cursor.advance(1);
        Ok(())
    }

    fn on_key(
        &mut self,
        cursor: &mut TokenCursor<'_>,
        _key: &str,
        _line: usize
    ) -> Result<(), GrammarError> {
        self.push_identation(0);
        self.push_current(cursor)?;
        Ok(())
    }

    fn after_key(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_whitespace()?;
        self.push_whitespaces(1);
        Ok(())
    }

    fn consume_equal(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        self.push_current(cursor)?;
        self.push_whitespaces(1);
        cursor.advance(1);
        cursor.skip_whitespace()?;
        Ok(())
    }

    fn enter_struct(&mut self) -> Result<(), GrammarError> {
        self.identation_level += 1;
        Ok(())
    }

    fn exit_struct(&mut self) -> Result<(), GrammarError> {
        self.identation_level -= 1;
        Ok(())
    }

    fn after_entry(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_just_whitespace()?;

        if let Some(SpannedToken { kind: Token::Comment, .. }) = cursor.tokens.get(cursor.position) {
            self.push_whitespaces(1);
            self.push_current(cursor)?;
            cursor.advance(1);
        } else if
            let Some(SpannedToken { kind: Token::Ident, .. }) = cursor.tokens.get(cursor.position)
        {
            self.push_line();
        }
        Ok(())
    }

    fn on_string(&mut self, lexeme: &str) -> Result<(), GrammarError> {
        self.formatted_tokens.push(FormatToken::new(Token::String, lexeme.to_string()));
        Ok(())
    }

    fn on_ident(&mut self, lexeme: &str) -> Result<(), GrammarError> {
        match lexeme {
            "true" | "false" => {
                self.formatted_tokens.push(FormatToken::new(Token::Ident, lexeme.to_string()));
            }
            _ => {
                self.formatted_tokens.push(FormatToken::new(Token::String, lexeme.to_string()));
            }
        }
        Ok(())
    }

    fn on_int(&mut self, lexeme: &str, token: &SpannedToken) -> Result<(), GrammarError> {
        let value = lexeme.parse::<i64>().map_err(|_| GrammarError::InvalidNumber {
            value: lexeme.to_string(),
            token: token.clone(),
        })?;
        self.formatted_tokens.push(FormatToken::new(Token::Number, value.to_string()));
        Ok(())
    }

    fn on_float(&mut self, lexeme: &str, token: &SpannedToken) -> Result<(), GrammarError> {
        let value = lexeme.parse::<f64>().map_err(|_| GrammarError::InvalidNumber {
            value: lexeme.to_string(),
            token: token.clone(),
        })?;
        self.formatted_tokens.push(FormatToken::new(Token::Number, value.to_string()));
        Ok(())
    }

    fn consume_array_open(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        self.push_current(cursor)?;
        cursor.advance(1);
        cursor.skip_whitespace()?;
        Ok(())
    }

    fn expect_comma(&mut self, cursor: &mut TokenCursor<'_>) -> Result<bool, GrammarError> {
        if cursor.in_bounds() && cursor.current()?.kind == Token::Comma {
            self.push_current(cursor)?;
            cursor.advance(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn after_array_comma(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        cursor.skip_whitespace()?;
        if cursor.current()?.kind == Token::CloseSquare {
            self.formatted_tokens.pop();
        } else {
            self.push_whitespaces(1);
        }
        Ok(())
    }

    fn consume_array_close(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        self.push_current(cursor)?;
        cursor.advance(1);
        Ok(())
    }
}

pub struct Formatter<'a> {
    source: &'a str,
    tokens: &'a [SpannedToken],
}

impl<'a> Formatter<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Self { source, tokens }
    }

    pub fn format(self) -> Result<List, GrammarError> {
        let mut grammar = Grammar::new(
            self.source,
            self.tokens,
            FormatterHandler::new(self.source, self.tokens.len())
        );
        grammar.run()?;
        Ok(grammar.handler.into_list())
    }
}

pub fn format<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> Result<List, GrammarError> {
    Formatter::new(source, tokens).format()
}
