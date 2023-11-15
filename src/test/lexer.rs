use crate::ast::{Expr, Greek, Operator, ID};
use crate::test::utils::expr_test_runner;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator);

#[test]
fn id_test() {
    // Int test
    assert_eq!(calculator::IntParser::new().parse("22"), Ok(22));
    assert_eq!(calculator::FloatParser::new().parse("22.22"), Ok(22.22));

    // ID test
    let id_map = vec![
        ("a", ID::ASCII('a')),
        ("e", ID::E),
        ("\\pi", ID::Pi),
        ("\\alpha", ID::Greek(Greek::Alpha)),
        ("\\beta", ID::Greek(Greek::Beta)),
        ("\\gamma", ID::Greek(Greek::Gamma)),
    ];

    id_map
        .into_iter()
        .for_each(|(k, v)| assert_eq!(calculator::IdParser::new().parse(k), Ok(v)));
}

// #[test]
// fn assign_test() {
//     expr_test_runner(
//         vec![
//             (
//                 r"a = b",
//                 Expr::Operation {
//                     l: Box::new(Expr::Id(ID::ASCII('a'))),
//                     r: Box::new(Expr::Id(ID::ASCII('b'))),
//                     opt: Operator::Assign,
//                 },
//             ),
//             (
//                 r"{(a)} = {(b)}",
//                 Expr::Operation {
//                     l: Box::new(Expr::Id(ID::ASCII('a'))),
//                     r: Box::new(Expr::Id(ID::ASCII('b'))),
//                     opt: Operator::Assign,
//                 },
//             ),
//         ]
//     )
// }
