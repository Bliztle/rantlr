program : rule SEMI program |;
rule : rule' SEMI;
rule' : rule'' rule''';
rule'' : TERMINAL rule'' | NONTERMINAL rule'' |;
rule''' : BAR rule'|;

TERMINAL: aa*;
NONTERMINAL: bb*;
