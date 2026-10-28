use crate::{ commands::Error, file::open, parser::{ List, parse }, tokenizer::tokenize };

pub fn list(path: &str) -> Result<List, Error> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    let list = parse(&contents, &tokens)?;
    Ok(list)
}
