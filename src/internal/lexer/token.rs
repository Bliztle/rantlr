#[derive(Debug, PartialEq)]
pub enum TokenKind {
    AlphaNumeric(String),
    OpenParen,
    CloseParen,
    Star,
    Bar,
    Semicolon,
    Colon,
    Equal,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub row: usize,
    pub col: usize,
}
