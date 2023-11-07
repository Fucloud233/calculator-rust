use std::collections::HashMap;
use lalrpop_util::lalrpop_mod;
use crate::ast::{Expr, ID, Rome};

lalrpop_mod!(pub calculator);

#[test]
fn exp_test() {
    // Int test
    assert!(calculator::IntParser::new().parse("22").is_ok());
    assert_eq!(calculator::ExprParser::new().parse("22"), Ok(Expr::Int(22)));

    // ID test
    let id_map: HashMap<&str, ID> = [
        ("a", ID::ASCII('a')),
        ("e", ID::E),
        ("\\pi", ID::Pi),
        ("\\alpha", ID::Rome(Rome::Alpha)),
        ("\\beta", ID::Rome(Rome::Beta)),
        ("\\gamma", ID::Rome(Rome::Gamma))
    ].into_iter().collect();

    id_map.into_iter().for_each(|(k, v)| 
        assert_eq!(calculator::IdParser::new().parse(k), Ok(v))
    );
}