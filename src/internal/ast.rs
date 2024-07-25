use crate::ast::Annotation;

struct Grammer {
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

impl From<Vec<Rule>> for Grammer {
    fn from(rules: Vec<Rule>) -> Self {
        Grammer { rules }
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
