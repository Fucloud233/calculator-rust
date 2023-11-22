// 表示一行输入，可以是表达式（Expression）或语句（Sentence，包含一个标识符 ID 和表达式 Expr
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
    Int(i64),
    Float(f64),
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
    Assign,
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
