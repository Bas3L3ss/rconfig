use std::fmt;

use thiserror::Error;

use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line(pub usize);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
    #[error(
        "unexpected token at line {line}: found {found:?}, expected {expected:?}"
    )] UnexpectedToken {
        found: Token,
        expected: Token,
        line: Line,
    },

    // #[error("unexpected token at line {1}: {0:?}")] Unexpected(Token, Line),

    #[error("expected expression at line {1}, found {0:?}")] ExpectedExpression(Token, Line),

    #[error("missing token at line {1}: expected {0:?}")] MissingToken(Token, Line),

    // #[error("unterminated string at line {0}")] UnterminatedString(Line),

    #[error("unterminated array at line {0}; expected `]`")] UnterminatedArray(Line),

    // #[error("unterminated struct at line {0}; expected `}}`")] UnterminatedStruct(Line),

    #[error("unexpected end of input at line {0}")] UnexpectedEof(Line),

    #[error("invalid number at line {1}: {0}")] InvalidNumber(String, Line),

    #[error("expected number after '-' at line {1}, found {0}")] InvalidSigned(String, Line),

    // #[error("duplicate key at line {1}: `{0}`")] DuplicateKey(String, Line),

    // #[error("expected key at line {1}, found {0:?}")] ExpectedKey(Token, Line),
}
