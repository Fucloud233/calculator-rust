use std::collections::HashMap;

use crate::utils::file;
use crate::ast::{ID, Expr, Line};

// TODO: 我自己需要完成的部分，在这里来实现符号表,并且完善相关函数
// COMMENT:结构定义 (Calculator):包含一个符号表 symbol_table，用于存储变量名（ID）和它们的值（f64）
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

    // COMMENT:接受一个字符串（表达式）并尝试计算其结果。它首先解析表达式，然后处理表达式。
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

    // COMMENT: 多行表达式，这里可以读入文件
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

// TODO：封装 LALRPOP 解析器接口，用于将输入行解析为 Line 枚举
// encapsulate the lalrpop interface
fn parse_line(line: &str) -> Result<Line, ()> {

    todo!()
}