use crate::ast::Annotation;

#[derive(Debug)]
pub struct Grammar {
    pub rules: Vec<Production>,
}

#[derive(Debug)]
pub enum Production {
    Parser(String, Vec<Vec<Ident>>),
    Lexer(String, String),
}

#[derive(Debug)]
pub enum Ident {
    Terminal(String),
    NonTerminal(String),
}

impl From<Vec<Production>> for Grammar {
    fn from(rules: Vec<Production>) -> Self {
        Grammar { rules }
    }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        if s.chars().next().unwrap().is_uppercase() {
            Ident::NonTerminal(s.to_string())
        } else {
            Ident::Terminal(s.to_string())
        }
    }
}

impl Annotation for Production {}
