use crate::ast::{Greek, ID};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

#[test]
// COMMENT:解析器测试：测试解析器是否能正确解析整数（如 "22"）和浮点数（如 "22.22"）
fn id_test() {
    // Int test
    assert_eq!(parser::IntParser::new().parse("22"), Ok(22));
    assert_eq!(parser::FloatParser::new().parse("22.22"), Ok(22.22));

    // ID test
    // COMMENT:测试不同类型的标识符，包括普通 ASCII 字符（如 'a'），特殊常数（如 'e' 和 'π'），以及希腊字母（如 'α', 'β', 'γ'）。
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
