use crate::ast::ID;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum CalculatorError<'input> {
    /// I don't know what will happen here when use 'static lifetime
    ParseError(ParseError<usize, Token<'input>, &'static str>),

    // you can define other types of error
    ArithmeticError(&'input str),
    // TODO: Overflow error needs to be done
    OverflowError,
    UndefinedIdError(ID),
    PrecisionError,
    UnusedExpressionError(String),
}

impl<'input> fmt::Display for CalculatorError<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::PrecisionError => write!(f, "Precision Error: the result may not be accurate"),
            CalculatorError::ParseError(e) => write!(f, "Parse Error: {}", e),
            CalculatorError::ArithmeticError(e) => write!(f, "Arithmetic Error: {}", e),
            CalculatorError::OverflowError => write!(f, "Overflow Error"),
            CalculatorError::UndefinedIdError(id) => write!(f, "Undefined Identifier Error: {:?}", id),
            CalculatorError::UnusedExpressionError(msg) => write!(f, "Unused Expression Error: {}", msg),
            // 其他错误类型...
        }
    }
}
