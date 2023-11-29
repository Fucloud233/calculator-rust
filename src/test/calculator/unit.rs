use crate::test::utils::{line_test_runner, new_value, new_operation};
use crate::ast::{ID, Line, Operator};

#[test]
fn test_parse_line() {
    line_test_runner(vec![
        ("1 + 1", Line::Expression(new_operation(new_value(1.), new_value(1.), Operator::Plus))),
        ("a = 1", Line::Sentence(ID::ASCII('a'), new_value(1.)))
    ]);
}