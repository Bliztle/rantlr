use crate::{
    ast::Node,
    internal::ast::{Grammar, Ident, Production},
};

use super::tree::{ParseR1, ParseR2, ParseR3, ParseR4, ParseS};

trait PushReturn<T> {
    fn push_return(self, item: T) -> Self;
}

impl<T> PushReturn<T> for Vec<T> {
    fn push_return(mut self, item: T) -> Self {
        self.push(item);
        self
    }
}

fn visit_parse_s(node: &Node<ParseS>) -> Vec<Production> {
    match &node.node {
        ParseS::Concat(production, next) => {
            visit_parse_s(next).push_return(visit_parse_r1(production))
        }
        ParseS::Epsilon => vec![],
    }
}

fn visit_parse_r1(node: &Node<ParseR1>) -> Production {
    match &node.node {
        ParseR1::Terminal(ident, pattern) => Production::Lexer(ident.into(), pattern.into()),
        ParseR1::NonTerminal(ident, rule) => Production::Parser(ident.into(), visit_parse_r2(rule)),
    }
}

fn visit_parse_r2(node: &Node<ParseR2>) -> Vec<Vec<Ident>> {
    match &node.node {
        ParseR2::Rule(rule, rules) => visit_parse_r4(rules).push_return(visit_parse_r3(rule)),
    }
}

fn visit_parse_r3(node: &Node<ParseR3>) -> Vec<Ident> {
    match &node.node {
        ParseR3::NonTerminal(ident, rest) => {
            visit_parse_r3(rest).push_return(Ident::NonTerminal(ident.into()))
        }
        ParseR3::Terminal(ident, rest) => {
            visit_parse_r3(rest).push_return(Ident::Terminal(ident.into()))
        }
        ParseR3::Epsilon => vec![],
    }
}

fn visit_parse_r4(node: &Node<ParseR4>) -> Vec<Vec<Ident>> {
    match &node.node {
        ParseR4::Concat(rest) => visit_parse_r2(rest),
        ParseR4::Epsilon => vec![],
    }
}

pub fn transform(tree: &Node<ParseS>) -> Grammar {
    Grammar {
        rules: visit_parse_s(tree),
    }
}

impl From<Node<ParseS>> for Grammar {
    fn from(value: Node<ParseS>) -> Self {
        transform(&value)
    }
}
