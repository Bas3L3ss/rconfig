use crate::{ file::{ FileError, open }, parser::parse, tokenizer::tokenize };

pub fn list(path: &str) -> Result<(), FileError> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    let list = parse(&contents, &tokens);

    println!("{list:#?}");
    Ok(())
}
