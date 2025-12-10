use thiserror::Error;

use crate::{
    error::{CompilerError, IntoCompilerError},
    grammar::{Span, Token, TokenType},
    lexer_base::LexError,
};

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    LexError(#[from] LexError),

    /// Expected a specific token, but found something else
    #[error("Expected {expected}, but found {found}")]
    UnexpectedToken { expected: String, found: String },

    /// Expected a specific token, but reached end of input
    #[error("Expected {expected}, but reached end of file")]
    UnexpectedEof { expected: String },
}

impl ParseError {
    /// Create an error for when we expected a specific token but hit EOF
    pub fn unexpected_eof<E: std::fmt::Display>(expected: E) -> Self {
        ParseError::UnexpectedEof {
            expected: expected.to_string(),
        }
    }

    pub fn unexpected_token<E: std::fmt::Display>(expected: E, found: &TokenType) -> Self {
        ParseError::UnexpectedToken {
            expected: expected.to_string(),
            found: found.to_string(),
        }
    }

    pub fn unexpected<E: std::fmt::Display>(expected: E, found: Option<&TokenType>) -> Self {
        match found {
            Some(found) => Self::unexpected_token(expected, found),
            None => Self::unexpected_eof(expected),
        }
    }
}

impl IntoCompilerError for ParseError {}
pub type ParseResult<T> = Result<T, CompilerError<ParseError>>;

impl CompilerError<ParseError> {
    pub fn from_peeked_token<S: std::fmt::Display>(
        expected: S,
        peeked_token: Option<Token>,
        eof_span: Span,
    ) -> Self {
        let parse_error = match peeked_token {
            Some(token) => ParseError::UnexpectedToken {
                expected: expected.to_string(),
                found: format!("{:?}", token),
            },
            None => ParseError::UnexpectedEof {
                expected: expected.to_string(),
            },
        };
        CompilerError::new(parse_error, eof_span)
    }
}
