use crate::{ file::{ FileError, open }, parser::parse, tokenizer::tokenize };

pub fn list(path: &str) -> Result<(), FileError> {
    let contents = open(path)?;
    // Tokenizing
    let tokens = tokenize(&contents);

    // Parsing
    let list = parse(&contents, &tokens);
    // Validating
    // Listing

    Ok(())
}
