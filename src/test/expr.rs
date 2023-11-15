use crate::ast::{Expr, Greek, Operator, ID};
use crate::test::utils::expr_test_runner;

// test expr parser
#[test]
fn expr_test() {
    expr_test_runner(
        vec![
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
    )
}

// test order between difference operation
#[test]
fn order_test() {
    expr_test_runner(
        vec![(
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
    )
}