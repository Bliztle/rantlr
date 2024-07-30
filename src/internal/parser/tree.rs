use crate::ast::Node;

#[derive(Debug)]
pub enum ParseS {
    Concat(Box<Node<ParseR1>>, Box<Node<ParseS>>),
    Epsilon,
}

#[derive(Debug)]
pub enum ParseR1 {
    NonTerminal(String, Box<Node<ParseR2>>),
    Terminal(String, String),
}

#[derive(Debug)]
pub enum ParseR2 {
    Rule(ParseR3, Box<Node<ParseR4>>),
}

#[derive(Debug)]
pub enum ParseR3 {
    NonTerminal(String, Box<Node<ParseR3>>),
    Terminal(String, Box<Node<ParseR3>>),
    Epsilon,
}

#[derive(Debug)]
pub enum ParseR4 {
    Concat(Box<Node<ParseR2>>),
    Epsilon,
}
