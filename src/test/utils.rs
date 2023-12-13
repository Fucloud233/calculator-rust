use crate::ast::{Expr, Greek, Line, Operator, UnaryOperator, ID};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

pub fn expr_test_runner(map: Vec<(&str, Expr)>) {
    map.into_iter()
        .for_each(|(k, v)| assert_eq!(parser::ExprParser::new().parse(k), Ok(Box::new(v))));
}

pub fn line_test_runner(map: Vec<(&str, Line)>) {
    map.into_iter()
        .for_each(|(k, v)| assert_eq!(parser::LineParser::new().parse(k), Ok(v)));
}

pub fn new_operation(l: Expr, r: Expr, opt: Operator) -> Expr {
    Expr::Operation {
        l: Box::new(l),
        r: Box::new(r),
        opt,
    }
}

pub fn new_unary_operation(operand: Expr, opt: UnaryOperator) -> Expr {
    Expr::UnaryOperation {
        operand: Box::new(operand),
        opt,
    }
}

pub fn new_arithmetic(l: f64, r: f64, opt: Operator) -> Expr {
    Expr::Operation {
        l: Expr::new_value(l),
        r: Expr::new_value(r),
        opt,
    }
}

pub fn new_value(c: f64) -> Expr {
    Expr::Value(c)
}

pub fn new_greek(c: Greek) -> Expr {
    Expr::Id(ID::Greek(c))
}

pub fn new_ascii(c: char) -> Expr {
    Expr::Id(ID::ASCII(c))
}
