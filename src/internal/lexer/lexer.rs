use crate::error::LexerError;

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
        .count();
    let matched = &data[..bytes_read];
    (matched, bytes_read)
}

struct Tokenizer<'a> {
    row: usize,
    col: usize,
    remaining_text: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer {
            row: 0,
            col: 0,
            remaining_text: value,
        }
    }
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

        let (kind, size) = match next {
            // Symbols
            '(' => (TokenKind::OpenParen, 1),
            ')' => (TokenKind::CloseParen, 1),
            '|' => (TokenKind::Bar, 1),
            ';' => (TokenKind::Semicolon, 1),
            ':' => (TokenKind::Colon, 1),
            '=' => (TokenKind::Equal, 1),
            '*' => (TokenKind::Star, 1),

            // Other
            c @ '_' | c if c.is_alphanumeric() => {
                let (id, length) = read_identifier(self.remaining_text);
                (TokenKind::AlphaNumeric(id.into()), length)
            }
            _ => return Err(LexerError::UnexpectedChar(next).into()),
        };

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

    Ok(tokens)
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

    test_next_token!(tokenize_single_open_brace, "(" => TokenKind::OpenParen);
    test_next_token!(tokenize_single_close_brace, ")" => TokenKind::CloseParen);
    test_next_token!(tokenize_single_star, "*" => TokenKind::Star);
    test_next_token!(tokenize_single_bar, "|" => TokenKind::Bar);
    test_next_token!(tokenize_single_semicolon, ";" => TokenKind::Semicolon);
    test_next_token!(tokenize_single_colon, ":" => TokenKind::Colon);
    test_next_token!(tokenize_single_equal, "=" => TokenKind::Equal);
    test_next_token!(tokenize_alpha_numeric, "abc" => TokenKind::AlphaNumeric("abc".into()));
    test_next_token!(tokenize_open_brace_with_trail, ")fdsfs" => TokenKind::CloseParen);

    test_tokenize!(
        tokenize_program_rule,
        "program ::= rule SEMI | rule SEMI program" => vec![
            token!(TokenKind::AlphaNumeric("program".into()), 0, 0),
            token!(TokenKind::Colon, 0, 8),
            token!(TokenKind::Colon, 0, 9),
            token!(TokenKind::Equal, 0, 10),
            token!(TokenKind::AlphaNumeric("rule".into()), 0, 12),
            token!(TokenKind::AlphaNumeric("SEMI".into()), 0, 17),
            token!(TokenKind::Bar, 0, 22),
            token!(TokenKind::AlphaNumeric("rule".into()), 0, 24),
            token!(TokenKind::AlphaNumeric("SEMI".into()), 0, 29),
            token!(TokenKind::AlphaNumeric("program".into()), 0, 34),
        ]
    );

    test_tokenize!(
        tokenize_program_rule_with_comments,
        "program ::= rule SEMI // comment\n| rule SEMI program" => vec![
            token!(TokenKind::AlphaNumeric("program".into()), 0, 0),
            token!(TokenKind::Colon, 0, 8),
            token!(TokenKind::Colon, 0, 9),
            token!(TokenKind::Equal, 0, 10),
            token!(TokenKind::AlphaNumeric("rule".into()), 0, 12),
            token!(TokenKind::AlphaNumeric("SEMI".into()), 0, 17),
            token!(TokenKind::Bar, 1, 0),
            token!(TokenKind::AlphaNumeric("rule".into()), 1, 2),
            token!(TokenKind::AlphaNumeric("SEMI".into()), 1, 7),
            token!(TokenKind::AlphaNumeric("program".into()), 1, 12),
        ]
    );
}
