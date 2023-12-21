use crate::ast::ID;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;


#[derive(Debug)]
pub struct CalculatorError<'input> {
    kind: CalculatorErrorKind<'input>,
    index: Option<usize>
}


use CalculatorErrorKind::*;

impl<'input> CalculatorError<'input> {
    pub fn new(kind: CalculatorErrorKind<'input>, index: Option<usize>) -> Self {
        CalculatorError {
            kind, index
        }
    }

    pub fn new_err<T>(kind: CalculatorErrorKind<'input>, index: Option<usize>) -> Result<T, Self> {
        Err(CalculatorError::new(kind, index))
    }
    pub fn message(&self) -> String {
        match &self.kind {
            ParseError(e) => format!("Parse Error: {}", e),
            ArithmeticError(e) => format!("Arithmetic Error: {}", e),
            OverflowError => format!("Overflow Error: Maximum allowed digit count is 15"),
            UndefinedIdError(id) => format!("Undefined Identifier Error: {:?}", id),
            UnusedExpressionError(msg) => format!("Unused Expression Error: {}", msg),
            NotValueReturn => String::from("Only expression is acceptable"),
            // 其他错误类型...
            _ => todo!()
        }
    } 
}

#[derive(PartialEq, Debug)]
pub enum CalculatorErrorKind<'input> {
    /// I don't know what will happen here when use 'static lifetime
    ParseError(ParseError<usize, Token<'input>, &'static str>),

    // you can define other types of error
    ArithmeticError(&'input str),
    // TODO: Overflow error needs to be done
    OverflowError,
    UndefinedIdError(ID),
    UnusedExpressionError(String),
    NotValueReturn,
}