#[derive(Debug, PartialEq)]
pub enum Line {
    Expression(Expr),
    Sentence(ID, Expr),
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

    UnaryOperation {
        operand: Box<Expr>,
        opt: UnaryOperator,
    },
}

impl Expr {
    pub fn new_value(value: f64) -> Box<Expr> {
        Box::new(Expr::Value(value))
    }

    pub fn new_operation(l: Box<Expr>, r: Box<Expr>, opt: Operator) -> Box<Expr> {
        Box::new(Expr::Operation { l, r, opt })
    }
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
pub enum UnaryOperator {
    Minus,
    Factorial,
    Sin,
    Cos,
    Tan,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ID {
    ASCII(char),
    Greek(Greek),
    Pi,
    E,
}

// Greek alphabet
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Greek {
    Alpha,
    Beta,
    Gamma,
}
