use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("Unsupported file extension: {0}")] UnsupportedExtension(String),

    #[error("Failed to read file '{path}' \n{source}")] Read {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("Failed to write file '{path}' \n{source}")] Write {
        path: String,
        #[source]
        source: io::Error,
    },
}
