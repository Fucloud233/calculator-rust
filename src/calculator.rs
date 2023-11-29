use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;

use crate::utils::error::CalculatorError;
use crate::ast::{ID, Expr, Line};

struct Calculator {
    symbol_table: HashMap<ID, f64>
}

// type ParseError = lalrpop_util::ParseError<usize, Token<'_>, &str>;

impl Calculator {

    /* --------------- init --------------- */

    pub fn new() -> Self {
        Calculator { symbol_table: HashMap::new() }
    }

    fn before_calculate(&mut self) {
        // you must clear the symbol table before clear
        self.symbol_table.clear();
    }


    /* --------------- calculator --------------- */

    // return error type to be determined
    pub fn calculate_expr<'input>(&mut self, line: &'input str) -> Result<f64, CalculatorError<'input>> {
        self.before_calculate();
        
        let parser_result = parse_line(line)?;

        if let Line::Expression(expr) = parser_result {
            self.handle_expression(&expr)
        } else {
            // an error will occur when parsing sentence
            todo!()
        }
    }

    // [Notice] reading and calculating must be decoupled
    // otherwise it will cause lifetime error
    pub fn calculate_file<'input>(&mut self, lines: Vec<&'input str>) -> Result<Vec<f64>, CalculatorError<'input>> {
        // init the status
        self.before_calculate();

        // using result array 
        // to decouple computation and output
        let mut results: Vec<f64> = Vec::new();
        for line in lines {
            // it will call line_parser to parse
            // which will return Line or custom error
            let parse_result: Line = parse_line(&line)?;
            match parse_result {
                Line::Expression(expr) => {
                    let value = self.handle_expression(&expr)?;
                    results.push(value);
                },
                Line::Sentence(id, expr) => self.handle_sentence(&id, &expr)?
            }
        };

        Ok(results)
    }

    /* --------------- handler --------------- */

    // TODO: expression will return value
    fn handle_expression<'input>(&mut self, expr: &Expr) -> Result<f64, CalculatorError<'input>> {
        todo!()
    }

    // TODO: sentence won't return value
    fn handle_sentence<'input>(&mut self, id: &ID, expr: &Expr) -> Result<(), CalculatorError<'input>> {
        todo!()
    }
}

/* --------------- parser --------------- */

lalrpop_mod!(pub parser);

// encapsulate the lalrpop interface
fn parse_line(line: &str) -> Result<Line, CalculatorError> {
    match parser::LineParser::new().parse(line) {
        Ok(r) => Ok(r), 
        Err(e) => return Err(CalculatorError::ParseError(e))
    }
}