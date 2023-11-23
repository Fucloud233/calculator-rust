use crate::ast::{Expr, Greek, Operator, ID};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

// COMMENT:表达式解析器的辅助函数和结构


// COMMENT:接受一组表达式和预期结果的对,使用 lalrpop_mod!(pub parser) 引入的解析器来解析每个表达式，并验证解析结果是否与预期相符。
pub fn expr_test_runner(map: Vec<(&str, Expr)>) {
    map.into_iter()
        .for_each(|(k, v)| assert_eq!(parser::ExprParser::new().parse(k), Ok(Box::new(v))));
}

// COMMENT:表达式构造，这是一个三元表达式，创建一个包含两个表达式和一个操作符的新 Expr::Operation
pub fn new_operation(l: Expr, r: Expr, opt: Operator) -> Expr {
    Expr::Operation { 
        l: Box::new(l), 
        r: Box::new(r), 
        opt 
    }
}


// COMMENT：辅助函数，关于创建新符号：分别创建代表希腊字符和 ASCII 字符的 Expr::Id
pub fn new_greek(c: Greek) -> Expr {
    Expr::Id(ID::Greek(c))
}

pub fn new_ascii(c: char) -> Expr {
    Expr::Id(ID::ASCII(c))
}