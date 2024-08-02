use crate::{
    ast::Node,
    error::ParserError,
    internal::lexer::token::{Token, TokenKind},
    unexpected_token,
};
use anyhow::Result;

use super::tree::{ParseR1, ParseR2, ParseR3, ParseR4, ParseS};

struct FunctionalParser<I: Iterator<Item = Token>> {
    remaining_input: I,
    next: Token,
}

/// Functional parser for the grammar.
///
/// Each parse rule only checks the relevant tokens, and only considers the nullability
/// of a nonterminal if it is explicitly nullable, to create a parse tree resembling
/// the grammar.
impl<I: Iterator<Item = Token>> FunctionalParser<I> {
    fn match_(&mut self, expected: &TokenKind) -> Result<()> {
        if self.next.kind == *expected {
            self.continue_()
        } else {
            unexpected_token!(self.next, expected.clone())
        }
    }
    fn continue_(&mut self) -> Result<()> {
        match self.remaining_input.next() {
            Some(next) => {
                self.next = next;
                Ok(())
            }
            None => Err(ParserError::UnexpectedEof.into()),
        }
    }

    fn parse_s(&mut self) -> Result<ParseS> {
        match self.next.kind {
            TokenKind::LexerIdent(_) | TokenKind::ParserIdent(_) => {
                let rule = self.parse_r1()?;
                self.match_(&TokenKind::Semicolon)?;
                let rest = self.parse_s()?;
                Ok(ParseS::Concat(rule.into(), rest.into()))
            }
            TokenKind::EOF => Ok(ParseS::Epsilon),
            _ => unexpected_token!(self.next, TokenKind::LexerIdent(String::new())),
        }
    }

    fn parse_r1(&mut self) -> Result<ParseR1> {
        match &self.next.kind {
            // Parser rule
            TokenKind::ParserIdent(ident) => {
                let ident_clone = ident.clone();
                self.continue_()?;
                self.match_(&TokenKind::Colon)?;
                let rule = self.parse_r2()?;
                Ok(ParseR1::NonTerminal(ident_clone, rule.into()))
            }
            // Lexer rule
            TokenKind::LexerIdent(ident) => {
                let ident_clone = ident.clone();
                self.continue_()?;
                self.match_(&TokenKind::Colon)?;
                match &self.next.kind {
                    TokenKind::LexerPattern(pattern) => {
                        let pattern_clone = pattern.clone();
                        self.continue_()?;
                        Ok(ParseR1::Terminal(ident_clone, pattern_clone))
                    }
                    _ => unexpected_token!(self.next, TokenKind::LexerPattern(String::new())),
                }
            }
            _ => unexpected_token!(self.next, TokenKind::ParserIdent(String::new())),
        }
    }

    /// This fx does not consider nullability, even though it is nullable,
    /// as it is not explicitly nullable, and thus is handled deeper in the tree.
    fn parse_r2(&mut self) -> Result<ParseR2> {
        let rule = self.parse_r3()?;
        let rest = self.parse_r4()?;
        Ok(ParseR2::Rule(rule.into(), rest.into()))
    }

    fn parse_r3(&mut self) -> Result<ParseR3> {
        match &self.next.kind {
            TokenKind::ParserIdent(ident) => {
                let ident_clone = ident.clone();
                self.continue_()?;
                let rest = self.parse_r3()?;
                Ok(ParseR3::NonTerminal(ident_clone, rest.into()))
            }
            TokenKind::LexerIdent(ident) => {
                let ident_clone = ident.clone();
                self.continue_()?;
                let rest = self.parse_r3();
                Ok(ParseR3::Terminal(ident_clone, rest?.into()))
            }
            TokenKind::Semicolon | TokenKind::Bar => Ok(ParseR3::Epsilon),
            _ => unexpected_token!(self.next, TokenKind::Semicolon),
        }
    }

    fn parse_r4(&mut self) -> Result<ParseR4> {
        match self.next.kind {
            TokenKind::Bar => {
                self.match_(&TokenKind::Bar)?;
                let rule = self.parse_r2()?;
                Ok(ParseR4::Concat(rule.into()))
            }
            TokenKind::Semicolon => Ok(ParseR4::Epsilon),
            _ => unexpected_token!(self.next, TokenKind::Semicolon),
        }
    }
}

pub fn parse(input: Vec<Token>) -> Result<Node<ParseS>> {
    let mut iter = input.into_iter();
    let first = iter.next();
    match first {
        None => Ok(ParseS::Epsilon.into()),
        Some(token) => {
            let parsed = FunctionalParser {
                remaining_input: iter,
                next: token,
            }
            .parse_s()?;

            Ok(parsed.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    impl From<TokenKind> for Token {
        fn from(value: TokenKind) -> Self {
            Token {
                kind: value,
                col: 0,
                row: 0,
            }
        }
    }

    macro_rules! test_parse_tokens_snapshot {
        ($name:ident, $src:expr) => {
            #[test]
            fn $name() {
                let kinds: Vec<TokenKind> = $src;
                let tokens: Vec<Token> = kinds.into_iter().map(|k| k.into()).collect();
                match parse(tokens) {
                    Ok(tree) => assert_debug_snapshot!(tree),
                    Err(err) => panic!("{err:?}"),
                }
            }
        };
    }

    // Below grammar is not LL(1)...
    // "program: rule SEMI | rule SEMI program;"
    test_parse_tokens_snapshot!(
        parse_program_production,
        vec![
            TokenKind::ParserIdent("program".into()),
            TokenKind::Colon,
            TokenKind::ParserIdent("rule".into()),
            TokenKind::LexerIdent("SEMI".into()),
            TokenKind::Bar,
            TokenKind::ParserIdent("rule".into()),
            TokenKind::LexerIdent("SEMI".into()),
            TokenKind::ParserIdent("program".into()),
            TokenKind::Semicolon,
            TokenKind::EOF
        ]
    );
}
