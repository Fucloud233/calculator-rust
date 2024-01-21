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

#[test]
fn test_handle_line() {
    let cases = vec![
        // FIXME: "1-2" can be parsed
        ("1+2", 3.),
        ("1- 2", -1.),
        ("1*2", 2.),
        ("1/2", 0.5),
        ("2^2", 4.),
        ("e^1", f64::consts::E),
        ("e^{-1}", f64::consts::E.recip()),
        ("\\sqrt[3]8", 2.),
        ("\\sqrt{16}", 4.),
        ("\\log_2 8", 3.),
        ("\\ln e", 1.),
        ("-(3+4)", -7.),
        ("(1+2)!", 6.),
        ("\\sin(\\pi / 2)", 1.),
        ("\\cos0", 1.),
        ("\\tan(\\pi / 4)", 1.),
        // extension test cases
        ("\\sin(\\pi/2) * 2", 2.),
        ("\\sin(\\pi/2)!", 1.),
        ("\\sin(\\sin(\\pi/2) * (\\pi/2))", 1.),
    ];

    let mut calculator = Calculator::new();
    cases.iter().for_each(|(line, value)| {
        match calculator.calculate_line(line, None) {
            Ok(Some(r)) => assert_eq!(r, *value),
            Ok(None) => {
                assert!(true)
            }
            Err(e) => {
                eprintln!("{}:\n{:#?}", line, e);
                assert!(false)
            }
        };
    })
}

#[test]
fn test_handle_expression() {
    let mut test_cal = Calculator::new();
    // insert a and b into symbol table
    let _ = test_cal.handle_sentence(&ID::ASCII('a'), &new_value(1.0));
    let _ = test_cal.handle_sentence(
        &ID::ASCII('b'),
        &new_operation(new_value(1.0), new_value(2.0), Operator::Plus),
    );

    let cases = vec![
        (
            new_operation(new_value(1.0), new_value(2.0), Operator::Plus),
            3.0,
        ),
        (
            new_operation(new_ascii('a'), new_value(2.0), Operator::Plus),
            3.0,
        ),
        (
            new_operation(new_ascii('a'), new_ascii('b'), Operator::Plus),
            4.0,
        ),
        (
            new_operation(new_value(1.0), new_value(2.0), Operator::Sub),
            -1.0,
        ),
        (
            new_operation(new_value(1.0), new_value(2.0), Operator::Mul),
            2.0,
        ),
        (
            new_operation(new_value(1.0), new_value(2.0), Operator::Div),
            0.5,
        ),
        (
            new_operation(new_value(2.0), new_value(2.0), Operator::Power),
            4.0,
        ),
        (
            new_operation(Expr::Id(ID::E), new_value(1.0), Operator::Power),
            f64::consts::E,
        ),
        (
            new_operation(Expr::Id(ID::E), new_value(-1.0), Operator::Power),
            f64::consts::E.recip(),
        ),
        (
            new_operation(new_value(3.0), new_value(8.0), Operator::Root),
            2.0,
        ),
        (
            new_operation(new_value(2.0), new_value(8.0), Operator::Log),
            3.0,
        ),
        (
            new_operation(Expr::Id(ID::E), Expr::Id(ID::E), Operator::Log),
            1.0,
        ),
    ];

    cases
        .into_iter()
        .for_each(|(k, v)| assert_eq!(test_cal.handle_expression(&k), Ok(v)))
}
