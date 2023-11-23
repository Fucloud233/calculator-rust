use crate::ast::{Expr, Operator, ID};
use crate::test::utils::expr_test_runner;

// COMMENT:包含用于测试高优先级运算（如幂运算、根号和对数）的单元测试
// test power
#[test]
// COMMENT:测试解析器是否能正确处理幂运算，包括普通的幂运算（如 "a ^ b"）、使用 LaTeX 语法的指数运算（如 "\exp 2"）
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
                // FIXME:是否缺少复杂表达式比如e(a*b)
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
// COMMENT:测试解析器对根号运算的处理，包括普通的平方根（如 "\sqrt 2"）和更复杂的根号表达式（如 "\sqrt[a]b"）
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
                // COMMENT:这里是b的a次根号运算
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
// COMMENT：测试不同形式的对数运算，如普通对数（如 "\log 2"）、自然对数（如 "\ln 2"）和更复杂的对数表达式（如 "\log_{(a)}{b}"）
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