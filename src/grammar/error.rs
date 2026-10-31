use thiserror::Error;

use crate::tokenizer::{ SpannedToken, Token };

#[derive(Debug, Error, PartialEq)]
pub enum GrammarError {
    #[error(
        "Unexpected token at line {}: expected {expected:?} found {:?}",
        found.line,
        found.kind
    )] UnexpectedToken {
        found: SpannedToken,
        expected: Token,
    },

    #[error(
        "Expected an expression at line {} found {:?}",
        found.line,
        found.kind
    )] ExpectedExpression {
        found: SpannedToken,
    },

    #[error(
        "Duplicate key `{key}` at line {}, first defined at line {}",
        token.line,
        first.line
    )] DuplicatedKey {
        key: String,
        token: SpannedToken,
        first: SpannedToken,
    },

    #[error("Expected {expected:?} at line {}", token.line)] MissingToken {
        expected: Token,
        token: SpannedToken,
    },

    #[error(
        "Unterminated array at line {}: expected `]` found {:?}",
        token.line,
        token.kind
    )] UnterminatedArray {
        token: SpannedToken,
    },

    #[error("Unterminated struct at line {}: expected `}}`", token.line)] UnterminatedStruct {
        token: SpannedToken,
    },

    #[error("Unexpected end of file")] UnexpectedEof {
        last: SpannedToken,
    },

    #[error(
        "Invalid number at line {}: `{value}` is not a valid number",
        token.line
    )] InvalidNumber {
        value: String,
        token: SpannedToken,
    },
}
