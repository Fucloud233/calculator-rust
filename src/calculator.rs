use std::collections::HashMap;

use crate::utils::file;
use crate::ast::{ID, Expr, Line};

struct Calculator {
    symbol_table: HashMap<ID, f64>
}

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
    pub fn calculate_expr(&mut self, line: &str) -> Result<f64, ()> {
        self.before_calculate();
        
        let parser_result = parse_line(line)?;

        if let Line::Expression(expr) = parser_result {
            self.handle_expression(&expr)
        } else {
            // an error will occur when parsing sentence
            Err(())
        }
    }

    pub fn calculate_file(&mut self, file_path: &str) -> Result<Vec<f64>, ()> {
        // init the status
        self.before_calculate();

        let lines = match file::read_lines(file_path) {
            Ok(l) => l,
            Err(_) => {
                // TODO: you may need to define custom error
                // rather than io error
                return Err(())
            }
        };

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
    fn handle_expression(&mut self, expr: &Expr) -> Result<f64, ()> {
        todo!()
    }

    // TODO: sentence won't return value
    fn handle_sentence(&mut self, id: &ID, expr: &Expr) -> Result<(), ()> {
        todo!()
    }
}

/* --------------- parser --------------- */

// encapsulate the lalrpop interface
fn parse_line(line: &str) -> Result<Line, ()> {

    todo!()
}