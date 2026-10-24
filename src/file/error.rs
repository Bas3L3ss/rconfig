use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("Unsupported file extension: {0}")] UnsupportedExtension(String),

    #[error("Failed to read file '{path}'")] Read {
        path: String,
        #[source]
        source: io::Error,
    },
}
