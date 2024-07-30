use crate::{error::LexerError, unexpected_char};

use super::token::{Token, TokenKind};
use anyhow::Result;

impl Token {
    fn new(kind: TokenKind, row: usize, col: usize) -> Self {
        Token { kind, row, col }
    }
}

fn read_identifier(data: &str) -> (&str, usize) {
    let mut has_seen_tick = false;
    let bytes_read = data
        .chars()
        .take_while(|&c| {
            if c == '\'' {
                has_seen_tick = true;
                return true;
            } else if has_seen_tick && c != '\'' {
                return false;
            }
            c.is_alphanumeric() || c == '_'
        })
        .fold(0, |acc, c| acc + c.len_utf8());
    let matched = &data[..bytes_read];
    (matched, bytes_read)
}

#[allow(clippy::unnecessary_wraps)]
fn read_lexer_pattern(data: &str) -> Result<(&str, usize)> {
    let mut err = None;
    let bytes_read = data
        .chars()
        // Ensure the pattern is valid. If it's not, return an error
        .scan(&mut err, |err, c| {
            if c.is_whitespace() {
                **err = Some(unexpected_char!(c));
                None
            } else {
                Some(c)
            }
        })
        .take_while(|&c| c != ';')
        .fold(0, |acc, c| acc + c.len_utf8());

    match err {
        Some(err) => err,
        None => Ok((&data[..bytes_read], bytes_read)),
    }
}

struct Tokenizer<'a> {
    state: TokenizerState,
    row: usize,
    col: usize,
    remaining_text: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer {
            state: TokenizerState::Initial,
            row: 0,
            col: 0,
            remaining_text: value,
        }
    }
}

/// States used to identify whether a lexer pattern is expected next
/// Easier than constructing it in the parser, as it massively simplifies the NFAs
#[derive(Debug, PartialEq)]
enum TokenizerState {
    /// Not currently working on any rule
    Initial,
    /// Fx working on a parser rule
    Other,
    /// Has seen name of lexer rule, now expecting a colon
    LexerRule,
    /// Has seen colon, now expecting a lexer pattern
    LexerPattern,
}

impl<'a> Tokenizer<'a> {
    fn skip(&mut self) {
        loop {
            let initial_length = self.remaining_text.len();
            // Skip whitespace
            self.remaining_text
                .chars()
                .take_while(|c| c.is_whitespace())
                .for_each(|c| self.advance_char(c));
            // Skip comments
            let pairs = [("//", "\n"), ("/*", "*/")];
            for (start, end) in pairs {
                let mut offset: usize = 0;
                if self.remaining_text.starts_with(start) {
                    self.remaining_text
                        .chars()
                        .take_while(|&c| {
                            if c == end.chars().nth(offset).unwrap() {
                                offset += 1;
                                if offset == end.len() {
                                    return false;
                                }
                            } else {
                                offset = 0;
                            };
                            true
                        })
                        .for_each(|c| self.advance_char(c));
                }
            }

            if initial_length == self.remaining_text.len() {
                break;
            }
        }
    }

    fn next(&mut self) -> Result<Option<Token>> {
        self.skip();

        if self.remaining_text.is_empty() {
            return Ok(None);
        };

        let next = self.remaining_text.chars().next().unwrap();

        let (kind, size) = match self.state {
            TokenizerState::LexerPattern => {
                let (pattern, length) = read_lexer_pattern(self.remaining_text)?;
                (TokenKind::LexerPattern(pattern.into()), length)
            }
            _ => match next {
                // Symbols
                '|' => (TokenKind::Bar, 1),
                ';' => (TokenKind::Semicolon, 1),
                ':' => (TokenKind::Colon, 1),

                // Other
                c @ '_' | c if c.is_alphanumeric() => {
                    let (id, length) = read_identifier(self.remaining_text);
                    if c.is_uppercase() {
                        (TokenKind::LexerIdent(id.into()), length)
                    } else {
                        (TokenKind::ParserIdent(id.into()), length)
                    }
                }
                _ => return unexpected_char!(next),
            },
        };

        // Track the current state
        match (&self.state, &kind) {
            (TokenizerState::Initial, TokenKind::LexerIdent(_)) => {
                self.state = TokenizerState::LexerRule;
            }
            (TokenizerState::LexerRule, TokenKind::Colon) => {
                self.state = TokenizerState::LexerPattern;
            }
            (_, TokenKind::Semicolon) => {
                self.state = TokenizerState::Initial;
            }
            _ => {
                self.state = TokenizerState::Other;
            }
        }

        let token = Token::new(kind, self.row, self.col);
        self.advance(size);
        Ok(Some(token))
    }

    fn advance(&mut self, amount: usize) {
        self.remaining_text = &self.remaining_text[amount..];
        self.col += amount;
    }

    fn advance_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.row += 1;
                self.col = 0;
            }
            '\t' => self.col += 4,
            _ => self.col += 1,
        }
        self.remaining_text = &self.remaining_text[c.len_utf8()..];
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer::from(src);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next()? {
        tokens.push(token);
    }

    if let Some(last) = tokens.last() {
        let mut eof = last.clone();
        eof.kind = TokenKind::EOF;
        tokens.push(eof);
        Ok(tokens)
    } else {
        Err(LexerError::UnexpectedEof.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_next_token {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let src: &str = $src;
                let should_be = $should_be;

                let mut tokenizer = Tokenizer::from(src);
                match tokenizer.next() {
                    Ok(Some(token)) => assert_eq!(token.kind, should_be, "Got {:?}", should_be),
                    _ => panic!("Tokenizing got error"),
                }
            }
        };
    }

    macro_rules! test_tokenize {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let src: &str = $src;
                let should_be = $should_be;

                let tokens = tokenize(src).unwrap();
                assert_eq!(tokens, should_be);
            }
        };
    }

    macro_rules! token {
        ($kind:expr, $row:expr, $col:expr) => {
            Token {
                kind: $kind,
                row: $row,
                col: $col,
            }
        };
    }

    test_next_token!(tokenize_single_bar, "|" => TokenKind::Bar);
    test_next_token!(tokenize_single_semicolon, ";" => TokenKind::Semicolon);
    test_next_token!(tokenize_single_colon, ":" => TokenKind::Colon);
    test_next_token!(tokenize_single_parser_ident, "abc" => TokenKind::ParserIdent("abc".into()));
    test_next_token!(tokenize_single_lexer_ident, "ABC" => TokenKind::LexerIdent("ABC".into()));
    test_next_token!(tokenize_single_open_brace_with_trail, "|fdsfs" => TokenKind::Bar);

    test_tokenize!(
        tokenize_parser_rule,
        "program: rule SEMI | rule SEMI program;" => vec![
            token!(TokenKind::ParserIdent("program".into()), 0, 0),
            token!(TokenKind::Colon, 0, 7),
            token!(TokenKind::ParserIdent("rule".into()), 0, 9),
            token!(TokenKind::LexerIdent("SEMI".into()), 0, 14),
            token!(TokenKind::Bar, 0, 19),
            token!(TokenKind::ParserIdent("rule".into()), 0, 21),
            token!(TokenKind::LexerIdent("SEMI".into()), 0, 26),
            token!(TokenKind::ParserIdent("program".into()), 0, 31),
            token!(TokenKind::Semicolon, 0, 38),
            token!(TokenKind::EOF, 0, 38),
        ]
    );

    test_tokenize!(
        tokenize_parser_rule_with_comments,
        "program: rule SEMI // comment\n| rule SEMI program;" => vec![
            token!(TokenKind::ParserIdent("program".into()), 0, 0),
            token!(TokenKind::Colon, 0, 7),
            token!(TokenKind::ParserIdent("rule".into()), 0, 9),
            token!(TokenKind::LexerIdent("SEMI".into()), 0, 14),
            token!(TokenKind::Bar, 1, 0),
            token!(TokenKind::ParserIdent("rule".into()), 1, 2),
            token!(TokenKind::LexerIdent("SEMI".into()), 1, 7),
            token!(TokenKind::ParserIdent("program".into()), 1, 12),
            token!(TokenKind::Semicolon, 1, 19),
            token!(TokenKind::EOF, 1, 19),
        ]
    );

    test_tokenize!(
        tokenize_lexer_rule,
        "LexerRule: [A-Z][a-zA-Z0-9_]*'*;" => vec![
            token!(TokenKind::LexerIdent("LexerRule".into()), 0, 0),
            token!(TokenKind::Colon, 0, 9),
            token!(TokenKind::LexerPattern("[A-Z][a-zA-Z0-9_]*'*".into()), 0, 11),
            token!(TokenKind::Semicolon, 0, 31),
            token!(TokenKind::EOF, 0, 31),
        ]
    );

    test_tokenize!(
        tokenize_parser_and_lexer_rule,
        "program: rule SEMI | rule SEMI program;\nLexerRule: [A-Z][a-zA-Z0-9_]*'*;" => vec![
            token!(TokenKind::ParserIdent("program".into()), 0, 0),
            token!(TokenKind::Colon, 0, 7),
            token!(TokenKind::ParserIdent("rule".into()), 0, 9),
            token!(TokenKind::LexerIdent("SEMI".into()), 0, 14),
            token!(TokenKind::Bar, 0, 19),
            token!(TokenKind::ParserIdent("rule".into()), 0, 21),
            token!(TokenKind::LexerIdent("SEMI".into()), 0, 26),
            token!(TokenKind::ParserIdent("program".into()), 0, 31),
            token!(TokenKind::Semicolon, 0, 38),
            token!(TokenKind::LexerIdent("LexerRule".into()), 1, 0),
            token!(TokenKind::Colon, 1, 9),
            token!(TokenKind::LexerPattern("[A-Z][a-zA-Z0-9_]*'*".into()), 1, 11),
            token!(TokenKind::Semicolon, 1, 31),
            token!(TokenKind::EOF, 1, 31),
        ]
    );
}
