struct Grammer {
    rules: Vec<Rule>,
}

enum Rule {
    Production(Ident, Vec<Vec<Ident>>),
}
enum Ident {
    Terminal(String),
    NonTerminal(String),
}
