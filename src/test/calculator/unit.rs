use crate::ast::{Expr, Line, Operator, ID};
use crate::calculator::Calculator;
use crate::test::utils::{line_test_runner, new_ascii, new_greek, new_operation, new_value};
use std::f64;

#[test]
fn test_parse_line() {
    line_test_runner(vec![
        (
            "1 + 1",
            Line::Expression(new_operation(new_value(1.), new_value(1.), Operator::Plus)),
        ),
        ("a = 1", Line::Sentence(ID::ASCII('a'), new_value(1.))),
    ]);
}

#[test]
fn test_handle_sentence() {
    // new a Calculator
    let mut test_cal = Calculator::new();

    let cases = vec![
        (ID::ASCII('a'), new_value(1.0)),
        (
            ID::ASCII('b'),
            new_operation(new_value(1.0), new_value(2.0), Operator::Plus),
        ),
    ];

    cases
        .into_iter()
        .for_each(|(id, expr)| assert_eq!(test_cal.handle_sentence(&id, &expr), Ok(())))
}
