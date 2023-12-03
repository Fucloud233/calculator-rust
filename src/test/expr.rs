use crate::ast::{Greek, Operator, UnaryOperator};
use crate::test::utils::{expr_test_runner, new_arithmetic, new_ascii, new_greek, new_operation};

use super::utils::{new_unary_operation, new_value};

// test expr parser
#[test]
fn expr_test() {
    let cases = vec![
        ("12+23", new_arithmetic(12., 23., Operator::Plus)),
        ("12-23", new_arithmetic(12., 23., Operator::Sub)),
        (
            "a + b",
            new_operation(new_ascii('a'), new_ascii('b'), Operator::Plus),
        ),
        (
            "a + b - c",
            new_operation(
                new_operation(new_ascii('a'), new_ascii('b'), Operator::Plus),
                new_ascii('c'),
                Operator::Sub,
            ),
        ),
        (
            "\\alpha - \\beta",
            new_operation(
                new_greek(Greek::Alpha),
                new_greek(Greek::Beta),
                Operator::Sub,
            ),
        ),
        (
            "-(3+4)",
            new_unary_operation(
                new_operation(new_value(3.), new_value(4.), Operator::Plus),
                UnaryOperator::Minus,
            ),
        ),
        (
            "3!",
            new_unary_operation(new_value(3.), UnaryOperator::Factorial),
        ),
        (
            "sin1",
            new_unary_operation(new_value(1.), UnaryOperator::Sin),
        ),
    ];

    expr_test_runner(cases);
}

// test order between difference operation
#[test]
fn order_test() {
    let cases = vec![(
        "a + b * c",
        new_operation(
            new_ascii('a'),
            new_operation(new_ascii('b'), new_ascii('c'), Operator::Mul),
            Operator::Plus,
        ),
    )];

    expr_test_runner(cases);
}
