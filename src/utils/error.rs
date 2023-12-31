use crate::ast::ID;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

#[derive(PartialEq, Debug)]
pub enum CalculatorError<'input> {
    /// I don't know what will happen here when use 'static lifetime
    ParseError(ParseError<usize, Token<'input>, &'static str>),

    // you can define other types of error
    ArithmeticError(&'input str),
    OverflowError,
    UndefinedIdError(ID),
}
