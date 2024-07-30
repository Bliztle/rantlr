#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    ParserIdent(String),
    LexerIdent(String),
    LexerPattern(String),
    Bar,
    Semicolon,
    Colon,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub row: usize,
    pub col: usize,
}
