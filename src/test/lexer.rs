use crate::ast::{Greek, ID};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

#[test]
fn id_test() {
    // Int test
    assert_eq!(parser::ValueParser::new().parse("22"), Ok(22.));
    assert_eq!(parser::ValueParser::new().parse("22.22"), Ok(22.22));

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
        .for_each(|(k, v)| assert_eq!(parser::IdParser::new().parse(k), Ok(v)));
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
