#[derive(Debug, PartialEq)]
pub enum Line {
    Expression(Expr),
    Sentence(ID, Expr)
}

/* remember it will be same enum in Exp and Opt
 * the former is formula
 * the latter is an operator
 */
#[derive(Debug, PartialEq)]
pub enum Expr {
    Id(ID),
    Value(f64),
    Operation {
        l: Box<Expr>,
        r: Box<Expr>,
        opt: Operator,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Sub,
    Mul,
    Div,
    Power,
    Root,
    Log,
}

#[derive(Debug, PartialEq)]
pub enum ID {
    ASCII(char),
    Greek(Greek),
    Pi,
    E,
}

// Greek alphabet
#[derive(Debug, PartialEq)]
pub enum Greek {
    Alpha,
    Beta,
    Gamma,
}
