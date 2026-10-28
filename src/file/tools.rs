use std::{ fs, path::Path };

use crate::file::FileError;

pub fn open(path: &str) -> Result<String, FileError> {
    let file_path = Path::new(path);

    match file_path.extension().and_then(|s| s.to_str()) {
        Some("rcfg") => {}
        Some(ext) => {
            return Err(FileError::UnsupportedExtension(ext.to_owned()));
        }
        None => {
            return Err(FileError::UnsupportedExtension(String::from("<no extension>")));
        }
    }

    fs::read_to_string(file_path).map_err(|source| FileError::Read {
        path: file_path.display().to_string(),
        source,
    })
}
pub fn write(path: &str, content: &str) -> Result<(), FileError> {
    let file_path = Path::new(path);

    match file_path.extension().and_then(|s| s.to_str()) {
        Some("rcfg") => {}
        Some(ext) => {
            return Err(FileError::UnsupportedExtension(ext.to_owned()));
        }
        None => {
            return Err(FileError::UnsupportedExtension(String::from("<no extension>")));
        }
    }

    fs::write(file_path, content).map_err(|source| FileError::Write {
        path: file_path.display().to_string(),
        source,
    })?;

    Ok(())
}
