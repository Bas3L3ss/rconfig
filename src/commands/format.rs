use crate::{ commands::Error, file::open, tokenizer::tokenize, formatter };

pub fn format(path: &str) -> Result<(), Error> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    let formatted_tokens = formatter::format(&contents, &tokens)?;
    Ok(())
}
