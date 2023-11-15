use crate::ast::{Expr, Greek, Operator, ID};
use crate::test::utils::expr_test_runner;

// test power
#[test]
fn power_test() {
    expr_test_runner(
        vec![
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
    )
}

#[test]
fn sqrt_test() {
    expr_test_runner(
        vec![
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
    )
}

#[test]
fn log_test() {
    expr_test_runner(
        vec![
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
    )
}