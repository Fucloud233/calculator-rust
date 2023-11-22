use crate::ast::{Expr, Greek, Operator, ID};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

pub fn expr_test_runner(map: Vec<(&str, Expr)>) {
    map.into_iter()
        .for_each(|(k, v)| assert_eq!(parser::ExprParser::new().parse(k), Ok(Box::new(v))));
}


pub fn new_operation(l: Expr, r: Expr, opt: Operator) -> Expr {
    Expr::Operation { 
        l: Box::new(l), 
        r: Box::new(r), 
        opt 
    }
}

pub fn new_greek(c: Greek) -> Expr {
    Expr::Id(ID::Greek(c))
}

pub fn new_ascii(c: char) -> Expr {
    Expr::Id(ID::ASCII(c))
}