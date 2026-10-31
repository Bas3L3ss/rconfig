use std::collections::{ HashMap, hash_map::Entry };

use crate::grammar::{ Grammar, GrammarError, GrammarHandler };
use crate::tokenizer::SpannedToken;

struct ValidatorHandler {
    token_maps: Vec<HashMap<String, SpannedToken>>,
}

impl ValidatorHandler {
    fn new() -> Self {
        Self {
            token_maps: Vec::new(),
        }
    }
}

impl GrammarHandler for ValidatorHandler {
    fn begin_block(&mut self, _open_token: crate::tokenizer::Token) -> Result<(), GrammarError> {
        self.token_maps.push(HashMap::new());
        Ok(())
    }

    fn end_block(&mut self) -> Result<(), GrammarError> {
        self.token_maps.pop();
        Ok(())
    }

    fn register_key(&mut self, key: &str, token: &SpannedToken) -> Result<(), GrammarError> {
        let linemap = self.token_maps.last_mut().expect("linemap for active block");
        match linemap.entry(key.to_string()) {
            Entry::Vacant(entry) => {
                entry.insert(token.clone());
                Ok(())
            }
            Entry::Occupied(entry) =>
                Err(GrammarError::DuplicatedKey {
                    key: key.to_string(),
                    token: token.clone(),
                    first: entry.get().clone(),
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
