use crate::{ commands::Error, file::open, formatter, tokenizer::{ serialize, tokenize } };

pub fn format(path: &str) -> Result<(), Error> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    let formatted_tokens = formatter::format(&contents, &tokens)?;
    let formatted_content = serialize(formatted_tokens);
    Ok(())
}
