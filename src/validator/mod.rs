use std::collections::{ HashMap, hash_map::Entry };

use crate::grammar::{ Grammar, GrammarError, GrammarHandler };
use crate::tokenizer::SpannedToken;

struct ValidatorHandler {
    linemaps: Vec<HashMap<String, usize>>,
}

impl ValidatorHandler {
    fn new() -> Self {
        Self {
            linemaps: Vec::new(),
        }
    }
}

impl GrammarHandler for ValidatorHandler {
    fn begin_block(&mut self, _open_token: crate::tokenizer::Token) -> Result<(), GrammarError> {
        self.linemaps.push(HashMap::new());
        Ok(())
    }

    fn end_block(&mut self) -> Result<(), GrammarError> {
        self.linemaps.pop();
        Ok(())
    }

    fn register_key(&mut self, key: &str, line: usize) -> Result<(), GrammarError> {
        let linemap = self.linemaps.last_mut().expect("linemap for active block");
        match linemap.entry(key.to_string()) {
            Entry::Vacant(entry) => {
                entry.insert(line);
                Ok(())
            }
            Entry::Occupied(entry) =>
                Err(GrammarError::DuplicatedKey {
                    key: key.to_string(),
                    line,
                    first_line: *entry.get(),
                }),
        }
    }
}

pub struct Validator<'a> {
    source: &'a str,
    tokens: &'a [SpannedToken],
}

impl<'a> Validator<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Self { source, tokens }
    }

    pub fn validate(self) -> Result<(), GrammarError> {
        let mut grammar = Grammar::new(self.source, self.tokens, ValidatorHandler::new());
        grammar.run()
    }
}

pub fn validate<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> Result<(), GrammarError> {
    Validator::new(source, tokens).validate()
}
