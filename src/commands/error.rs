use thiserror::Error;

use crate::{ file::FileError, parser::ParserError };

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)] File(#[from] FileError),

    #[error(transparent)] Parser(#[from] ParserError),
}
