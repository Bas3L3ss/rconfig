use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Unknown file format: {0}")] NotRconfig(String),

    #[error("Failed to open file: {0}")] IoOpen(#[from] io::Error),

    #[error("Not a valid file (may be a directory or unreadable format): {0}")] NotAFile(String),

    #[error("File not found")]
    NotFound,

    #[error("Unknown file error")]
    Unknown,
}
