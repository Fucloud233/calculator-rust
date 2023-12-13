use crate::ast::{Greek, ID};
use crate::utils::file::read_lines;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

#[test]
fn test_read_lines() {
    let file_path = "test.txt";
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "line 1\nline 2\nline 3").unwrap();

    let result = read_lines(file_path).unwrap();

    assert_eq!(result, vec!["line 1", "line 2", "line 3"]);

    std::fs::remove_file(file_path).unwrap();
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
