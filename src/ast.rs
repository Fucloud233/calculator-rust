/* remember it will be same enum in Exp and Opt
 * the former is formula
 * the latter is an operator
 */
#[derive(Debug, PartialEq)]
pub enum Expr {
    Id(ID),
    Int(i64),
    Float(i64),
    Op{
        first: Box<Expr>,
        second: Box<Expr>,
        opt: Opt
    },
}

#[derive(Debug, PartialEq)]
pub enum Opt {
    Plus, Sub, Mul, Div, Power, Root, Log
}

#[derive(Debug, PartialEq)]
pub enum ID {
    ASCII(char),
    Rome(Rome),
    Pi,
    E,
}

#[derive(Debug, PartialEq)]
pub enum Rome {
    Alpha,
    Beta,
    Gamma
}