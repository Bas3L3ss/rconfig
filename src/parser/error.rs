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
        "Unexpected token at line {line}: expected `{expected:?}`, found `{found:?}`"
    )] UnexpectedToken {
        found: Token,
        expected: Token,
        line: Line,
    },

    #[error("Expected an expression at line {1}, found `{0:?}`")] ExpectedExpression(Token, Line),

    #[error("Expected `{0:?}` at line {1}")] MissingToken(Token, Line),

    #[error(
        "Unterminated array at line {0}: expected `]` before the end of input"
    )] UnterminatedArray(Line),

    #[error("Unexpected end of file")] UnexpectedEof,

    #[error("Invalid number at line {1}: `{0}` is not a valid number")] InvalidNumber(String, Line),

    #[error("Expected a number after `-` at line {1}, found `{0}`")] InvalidSigned(String, Line),
}
