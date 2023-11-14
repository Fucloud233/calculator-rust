use crate::ast::{Expr, Greek, Operator, ID};
use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;

lalrpop_mod!(pub calculator);

#[test]
fn id_test() {
    // Int test
    assert_eq!(calculator::IntParser::new().parse("22"), Ok(22));
    assert_eq!(calculator::FloatParser::new().parse("22.22"), Ok(22.22));

    // ID test
    let id_map: HashMap<&str, ID> = [
        ("a", ID::ASCII('a')),
        ("e", ID::E),
        ("\\pi", ID::Pi),
        ("\\alpha", ID::Greek(Greek::Alpha)),
        ("\\beta", ID::Greek(Greek::Beta)),
        ("\\gamma", ID::Greek(Greek::Gamma)),
    ]
    .into_iter()
    .collect();

    id_map
        .into_iter()
        .for_each(|(k, v)| assert_eq!(calculator::IdParser::new().parse(k), Ok(v)));
}

fn expr_test_runner(map: HashMap<&str, Expr>) {
    map.into_iter()
        .for_each(|(k, v)| assert_eq!(calculator::ExprParser::new().parse(k), Ok(Box::new(v))));
}

// test expr parser
#[test]
fn expr_test() {
    expr_test_runner(
        [
            (
                "a + b",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::ASCII('a'))),
                    r: Box::new(Expr::Id(ID::ASCII('b'))),
                    opt: Operator::Plus,
                },
            ),
            (
                "a + b - c",
                Expr::Operation {
                    l: Box::new(Expr::Operation {
                        l: Box::new(Expr::Id(ID::ASCII('a'))),
                        r: Box::new(Expr::Id(ID::ASCII('b'))),
                        opt: Operator::Plus,
                    }),
                    r: Box::new(Expr::Id(ID::ASCII('c'))),
                    opt: Operator::Sub,
                },
            ),
            (
                "\\alpha - \\beta",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::Greek(Greek::Alpha))),
                    r: Box::new(Expr::Id(ID::Greek(Greek::Beta))),
                    opt: Operator::Sub,
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}

// test factor parser
#[test]
fn factor_test() {
    expr_test_runner(
        [
            (
                "\\frac\\alpha\\beta",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::Greek(Greek::Alpha))),
                    r: Box::new(Expr::Id(ID::Greek(Greek::Beta))),
                    opt: Operator::Div,
                },
            ),
            (
                "\\frac{(\\alpha)}{\\beta}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::Greek(Greek::Alpha))),
                    r: Box::new(Expr::Id(ID::Greek(Greek::Beta))),
                    opt: Operator::Div,
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}

// test order between difference operation
#[test]
fn order_test() {
    expr_test_runner(
        [(
            "a + b * c",
            Expr::Operation {
                l: Box::new(Expr::Id(ID::ASCII('a'))),
                r: Box::new(Expr::Operation {
                    l: Box::new(Expr::Id(ID::ASCII('b'))),
                    r: Box::new(Expr::Id(ID::ASCII('c'))),
                    opt: Operator::Mul,
                }),
                opt: Operator::Plus,
            },
        )]
        .into_iter()
        .collect(),
    )
}

// test power
#[test]
fn power_test() {
    expr_test_runner(
        [
            (
                "a ^ b",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::ASCII('a'))),
                    r: Box::new(Expr::Id(ID::ASCII('b'))),
                    opt: Operator::Power,
                },
            ),
            (
                r"2 ^ {3.2}",
                Expr::Operation {
                    l: Box::new(Expr::Int(2)),
                    r: Box::new(Expr::Float(3.2)),
                    opt: Operator::Power,
                },
            ),
            (
                r"\exp 2",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Int(2)),
                    opt: Operator::Power,
                },
            ),
            (
                r"\exp {2.3}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Float(2.3)),
                    opt: Operator::Power,
                },
            ),
            (
                r"\exp a",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Id(ID::ASCII('a'))),
                    opt: Operator::Power,
                },
            ),
            (
                r"\exp {(a)}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Id(ID::ASCII('a'))),
                    opt: Operator::Power,
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}

#[test]
fn sqrt_test() {
    expr_test_runner(
        [
            (
                r"\sqrt 2",
                Expr::Operation {
                    l: Box::new(Expr::Float(0.5)),
                    r: Box::new(Expr::Int(2)),
                    opt: Operator::Root,
                },
            ),
            (
                r"\sqrt {2.5}",
                Expr::Operation {
                    l: Box::new(Expr::Float(0.5)),
                    r: Box::new(Expr::Float(2.5)),
                    opt: Operator::Root,
                },
            ),
            (
                r"\sqrt {(a)}",
                Expr::Operation {
                    l: Box::new(Expr::Float(0.5)),
                    r: Box::new(Expr::Id(ID::ASCII('a'))),
                    opt: Operator::Root,
                },
            ),
            (
                r"\sqrt[a]b",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::ASCII('a'))),
                    r: Box::new(Expr::Id(ID::ASCII('b'))),
                    opt: Operator::Root,
                },
            ),
            (
                r"\sqrt[3]8",
                Expr::Operation {
                    l: Box::new(Expr::Int(3)),
                    r: Box::new(Expr::Int(8)),
                    opt: Operator::Root,
                },
            ),
            (
                r"\sqrt[{(a)}]{(b)}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::ASCII('a'))),
                    r: Box::new(Expr::Id(ID::ASCII('b'))),
                    opt: Operator::Root,
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}

#[test]
fn log_test() {
    expr_test_runner(
        [
            (
                r"\log 2",
                Expr::Operation {
                    l: Box::new(Expr::Int(10)),
                    r: Box::new(Expr::Int(2)),
                    opt: Operator::Log,
                },
            ),
            (
                r"\log {2.5}",
                Expr::Operation {
                    l: Box::new(Expr::Int(10)),
                    r: Box::new(Expr::Float(2.5)),
                    opt: Operator::Log,
                },
            ),
            (
                r"\log {(a)}",
                Expr::Operation {
                    l: Box::new(Expr::Int(10)),
                    r: Box::new(Expr::Id(ID::ASCII('a'))),
                    opt: Operator::Log,
                },
            ),
            (
                r"\ln 2",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Int(2)),
                    opt: Operator::Log,
                },
            ),
            (
                r"\ln {2.5}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Float(2.5)),
                    opt: Operator::Log,
                },
            ),
            (
                r"\ln {(a)}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::E)),
                    r: Box::new(Expr::Id(ID::ASCII('a'))),
                    opt: Operator::Log,
                },
            ),
            (
                r"\log_{(a)}{b}",
                Expr::Operation {
                    l: Box::new(Expr::Id(ID::ASCII('a'))),
                    r: Box::new(Expr::Id(ID::ASCII('b'))),
                    opt: Operator::Log,
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}
