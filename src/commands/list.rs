use crate::file::{ FileError, open };

pub fn list(path: &str) -> Result<(), FileError> {
    let contents = open(path)?;

    // Tokenizing
    // Parsing
    // Validating
    // Listing

    Ok(())
}
