use thiserror::Error;

use crate::{
    file::FileError,
    formatter::FormatterError,
    parser::ParserError,
    validator::ValidatorError,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)] File(#[from] FileError),
    #[error(transparent)] Parser(#[from] ParserError),
    #[error(transparent)] Validator(#[from] ValidatorError),
    #[error(transparent)] Foramtter(#[from] FormatterError),
}
