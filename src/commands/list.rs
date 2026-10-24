use crate::{ file::{ FileError, open }, tokenizer::tokenize };

pub fn list(path: &str) -> Result<(), FileError> {
    let contents = open(path)?;
    // Tokenizing
    let tokens = tokenize(&contents);
    dbg!(tokens);

    // Parsing
    // Validating
    // Listing

    Ok(())
}
