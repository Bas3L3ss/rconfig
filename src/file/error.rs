use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("unsupported file extension: {0}")] UnsupportedExtension(String),

    #[error("failed to read file '{path}'")] Read {
        path: String,
        #[source]
        source: io::Error,
    },
}
