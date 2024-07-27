use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("Unexpected end of input")]
    UnexpectedEof,
}

// Err(LexerError::UnexpectedChar(next).into())

#[macro_export]
macro_rules! unexpected_char {
    ($c:expr) => {
        Err(LexerError::UnexpectedChar($c).into())
    };
}
