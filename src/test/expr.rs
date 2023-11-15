use crate::ast::{Greek, Operator};
use crate::test::utils::{expr_test_runner, new_operation, new_ascii, new_greek};

// test expr parser
#[test]
fn expr_test() {

    let cases = vec![(
        "a + b",
        new_operation(
            new_ascii('a'),
            new_ascii('b'),
            Operator::Plus
        )
    ),(
        "a + b - c",
        new_operation(
            new_operation(
                new_ascii('a'),
                new_ascii('b'),
                Operator::Plus
            ),
            new_ascii('c'),
            Operator::Sub
        )
    ),(
        "\\alpha - \\beta",
        new_operation(
            new_greek(Greek::Alpha),
            new_greek(Greek::Beta),
            Operator::Sub
        )
    )];
    
    expr_test_runner(cases);
}

// test order between difference operation
#[test]
fn order_test() {
    let cases = vec![(
        "a + b * c",
        new_operation(
            new_ascii('a'),
            new_operation(
                new_ascii('b'),
                new_ascii('c'), 
                Operator::Mul
            ),
            Operator::Plus
        )
    )];

    expr_test_runner(cases);
}