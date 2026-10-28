use crate::tokenizer::{ FormatToken, Token };

pub fn serialize(tokens: Vec<FormatToken>) -> String {
    let mut out = String::new();

    for token in tokens {
        match token.kind {
            Token::String => {
                out += format!("\"{}\"", token.content).as_str();
            }
            _ => {
                out += &token.content;
            }
        }
    }

    println!("{out}");

    out
}
