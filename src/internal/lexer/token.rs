#[derive(Debug, PartialEq)]
pub enum TokenKind {
    ParserIdent(String),
    LexerIdent(String),
    LexerPattern(String),
    Bar,
    Semicolon,
    Colon,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub row: usize,
    pub col: usize,
}
