use thiserror::Error;

use crate::{file::FileError, grammar::GrammarError};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    File(#[from] FileError),
    #[error(transparent)]
    Grammar(#[from] GrammarError),
}
