use crate::{ commands::Error, file::open, tokenizer::tokenize, validator };

pub fn validate(path: &str) -> Result<(), Error> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    validator::validate(&contents, &tokens)?;
    Ok(())
}
