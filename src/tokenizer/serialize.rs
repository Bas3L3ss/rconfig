use crate::tokenizer::{ FormatToken, Token };

pub fn serialize<'a>(tokens: Vec<FormatToken>) -> String {
    let mut out = String::new();

    for token in tokens {
        match token.kind {
            Token::String => {
                out += "\"";
                out += &token.content;
                out += "\"";
            }
            _ => {
                out += &token.content;
            }
        }
    }

    out
}
