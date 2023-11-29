use std::io::Error as IO_Error;
use lalrpop_util::ParseError;
use lalrpop_util::lexer::Token;


pub enum CalculatorError<'input>{
    IOError(IO_Error),
    /// I don't know what will happen here when use 'static lifetime
    ParseError(ParseError<usize, Token<'input>, &'static str>),

    // you can define other types of error
}