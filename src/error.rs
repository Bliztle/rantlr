use thiserror::Error;

use crate::internal::lexer::token::{Token, TokenKind};

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("Unexpected end of input")]
    UnexpectedEof,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}. Expected token of kind: {1:?}")]
    UnexpectedToken(Token, TokenKind),
    #[error("Unexpected end of input")]
    UnexpectedEof,
}

#[macro_export]
macro_rules! unexpected_char {
    ($c:expr) => {
        Err(LexerError::UnexpectedChar($c).into())
    };
}

#[macro_export]
macro_rules! unexpected_token {
    ($got:expr, $expected:expr) => {
        Err(ParserError::UnexpectedToken($got.clone(), $expected).into())
    };
}
