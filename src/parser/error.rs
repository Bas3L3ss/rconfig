use thiserror::Error;

use crate::tokenizer::Token;

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
    #[error(
        "Unexpected token at line {line}: expected {expected:?} found {found:?}"
    )] UnexpectedToken {
        found: Token,
        expected: Token,
        line: usize,
    },

    #[error("Expected an expression at line {line} found {found:?}")] ExpectedExpression {
        found: Token,
        line: usize,
    },

    #[error(
        "Duplicate key `{key}` at line {line}, first defined at line {first_line}"
    )] DuplicatedKey {
        key: String,
        line: usize,
        first_line: usize,
    },

    #[error("Expected {expected:?} at line {line}")] MissingToken {
        expected: Token,
        line: usize,
    },

    #[error("Unterminated array at line {line}: expected `]`")] UnterminatedArray {
        line: usize,
    },

    #[error("Unterminated struct at line {line}: expected `}}`")] UnterminatedStruct {
        line: usize,
    },

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Invalid number at line {line}: `{value}` is not a valid number")] InvalidNumber {
        value: String,
        line: usize,
    },
}
