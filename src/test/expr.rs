use crate::ast::{Greek, Operator};
use crate::test::utils::{expr_test_runner, new_operation, new_ascii, new_greek};

// test expr parser
#[test]
// COMMENT:测试解析器是否能正确处理简单的数学表达式
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
        // COMMENT:特别测试了希腊字母表达式（例如 "\alpha - \beta"），以确保解析器能处理特殊字符
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
// COMMENT:测试解析器是否能正确理解和应用操作符的优先级
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