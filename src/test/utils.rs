use crate::ast::Expr;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator);

pub fn expr_test_runner(map: Vec<(&str, Expr)>) {
    map.into_iter()
        .for_each(|(k, v)| assert_eq!(calculator::ExprParser::new().parse(k), Ok(Box::new(v))));
}