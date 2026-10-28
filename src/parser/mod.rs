pub mod value;
use std::collections::{ HashMap, hash_map::Entry };

pub use value::*;

use crate::grammar::{ Grammar, GrammarError, GrammarHandler, TokenCursor };
use crate::tokenizer::SpannedToken;

pub type List = HashMap<String, Value>;

struct ParserHandler {
    maps: Vec<List>,
    linemaps: Vec<HashMap<String, usize>>,
    values: Vec<Value>,
    array_starts: Vec<usize>,
}

impl ParserHandler {
    fn new() -> Self {
        Self {
            maps: Vec::new(),
            linemaps: Vec::new(),
            values: Vec::new(),
            array_starts: Vec::new(),
        }
    }

    fn into_list(mut self) -> List {
        self.maps.pop().expect("parser must produce a root map")
    }
}

impl GrammarHandler for ParserHandler {
    fn begin_block(&mut self, _open_token: crate::tokenizer::Token) -> Result<(), GrammarError> {
        self.maps.push(HashMap::new());
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

    fn exit_struct(&mut self) -> Result<(), GrammarError> {
        let map = self.maps.pop().expect("struct map");
        self.values.push(Value::Struct(map));
        Ok(())
    }

    fn finish_entry(&mut self, key: &str, _line: usize) -> Result<(), GrammarError> {
        let value = self.values.pop().expect("entry value");
        self.maps.last_mut().expect("active block map").insert(key.to_string(), value);
        Ok(())
    }

    fn on_string(&mut self, lexeme: &str) -> Result<(), GrammarError> {
        self.values.push(Value::String(lexeme.to_string()));
        Ok(())
    }

    fn on_ident(&mut self, lexeme: &str) -> Result<(), GrammarError> {
        match lexeme {
            "true" => self.values.push(Value::Bool(true)),
            "false" => self.values.push(Value::Bool(false)),
            _ => self.values.push(Value::String(lexeme.to_string())),
        }
        Ok(())
    }

    fn on_int(&mut self, lexeme: &str, line: usize) -> Result<(), GrammarError> {
        let value = lexeme.parse::<i64>().map_err(|_| GrammarError::InvalidNumber {
            value: lexeme.to_string(),
            line,
        })?;
        self.values.push(Value::Int(value));
        Ok(())
    }

    fn on_float(&mut self, lexeme: &str, line: usize) -> Result<(), GrammarError> {
        let value = lexeme.parse::<f64>().map_err(|_| GrammarError::InvalidNumber {
            value: lexeme.to_string(),
            line,
        })?;
        self.values.push(Value::Float(value));
        Ok(())
    }

    fn consume_array_open(&mut self, cursor: &mut TokenCursor<'_>) -> Result<(), GrammarError> {
        self.array_starts.push(self.values.len());
        cursor.advance(1);
        Ok(())
    }

    fn finish_array(&mut self) -> Result<(), GrammarError> {
        let start = self.array_starts.pop().expect("array start marker");
        let items = self.values.split_off(start);
        self.values.push(Value::Vec(items));
        Ok(())
    }
}

pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a [SpannedToken],
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a [SpannedToken]) -> Self {
        Parser { source, tokens }
    }

    pub fn parse(self) -> Result<List, GrammarError> {
        let mut grammar = Grammar::new(self.source, self.tokens, ParserHandler::new());
        grammar.run()?;
        Ok(grammar.handler.into_list())
    }
}

pub fn parse<'a>(source: &'a str, tokens: &'a [SpannedToken]) -> Result<List, GrammarError> {
    Parser::new(source, tokens).parse()
}
