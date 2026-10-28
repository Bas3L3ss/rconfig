use thiserror::Error;

use crate::tokenizer::Token;

#[derive(Debug, Error, PartialEq)]
pub enum GrammarError {
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
}
