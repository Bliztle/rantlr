use crate::ast::Annotation;

pub struct Grammar {
    rules: Vec<Rule>,
}

enum Rule {
    Parser(String, Vec<Vec<Ident>>),
    Lexer(String, String),
}
enum Ident {
    Terminal(String),
    NonTerminal(String),
}

impl From<Vec<Rule>> for Grammar {
    fn from(rules: Vec<Rule>) -> Self {
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

impl Annotation for Rule {}
