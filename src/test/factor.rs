use crate::ast::{Expr, Greek, Operator, ID};
use crate::test::utils::{expr_test_runner, new_arithmetic};


// test factor parser
#[test]
fn factor_test() {
    expr_test_runner(
        vec![
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
            ("\\frac12", new_arithmetic(1., 2., Operator::Div))
        ]
    )
}