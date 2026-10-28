use crate::commands::{ Error, list };

pub fn format(path: &str) -> Result<(), Error> {
    let list = list(&path)?;
    Ok(())
}
