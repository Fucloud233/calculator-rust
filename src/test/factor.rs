use crate::ast::{Expr, Greek, Operator, ID};
use crate::test::utils::expr_test_runner;


// COMMENT:用于验证解析器在处理分数（即除法）表达式时的正确性
// test factor parser
#[test]
// COMMENT:测试解析 LaTeX 语法的分数表达式，如 "\frac\alpha\beta"，确保解析器能正确将其解析为 α 除以 β。
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
        ]
    )
}