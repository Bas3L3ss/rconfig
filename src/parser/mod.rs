pub mod value;
use std::collections::HashMap;

pub use value::*;

use crate::tokenizer::{ SpannedToken, Token };

pub fn parse<'a>(source: &'a str, tokens: &[SpannedToken]) -> HashMap<&'a str, Value<'a>> {
    let list = HashMap::new();
    let mut cur = 0;

    align_ident(&mut cur, tokens);
    skip_whitespace(&mut cur, tokens);
    while cur < tokens.len() {
        match tokens[cur].kind {
            Token::Equal => {
                todo!();
            }
            Token::OpenBracket => {
                todo!();
            }
            _ => {
                todo!("Incorrect asdasdas");
            }
        }
    }
    list
}

fn parse_struct<'a>() -> Value<'a> {
    todo!()
}
fn assignement<'a>() -> Value<'a> {
    todo!()
}

fn align_ident<'a>(cur: &mut usize, tokens: &[SpannedToken]) {
    while *cur < tokens.len() {
        match &tokens[*cur].kind {
            Token::Ident => {
                break;
            }
            _ => {
                *cur += 1;
            }
        }
    }
}
fn skip_whitespace<'a>(cur: &mut usize, tokens: &[SpannedToken]) {
    while *cur < tokens.len() {
        match &tokens[*cur].kind {
            Token::WhiteSpace | Token::Newline => {
                *cur += 1;
            }
            _ => {
                break;
            }
        }
    }
}
