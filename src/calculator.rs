use statrs::function::factorial;
use std::collections::HashMap;
use std::f64;

use lalrpop_util::lalrpop_mod;

use crate::ast::{Expr, Line, Operator, UnaryOperator, ID};
use crate::utils::error::{
    CalculatorError,
    CalculatorErrorKind::{self, *},
};

pub struct Calculator {
    symbol_table: HashMap<ID, f64>,
}

// type ParseError = lalrpop_util::ParseError<usize, Token<'_>, &str>;

impl Calculator {
    /* --------------- init --------------- */

    pub fn new() -> Self {
        Calculator {
            symbol_table: HashMap::new(),
        }
    }

    #[inline]
    fn before_calculate(&mut self) {
        // you must clear the symbol table before clear
        self.symbol_table.clear();
    }

    /* --------------- calculator --------------- */

    pub fn calculate_expr<'input>(
        &mut self,
        line: &'input str,
    ) -> Result<f64, CalculatorError<'input>> {
        self.before_calculate();

        let parser_result = parse_line(line)?;

        match parser_result {
            Line::Expression(expr) => {
                let result = self
                    .handle_expression(&expr)
                    .map_err(|kind| CalculatorError::new(kind, None))?;

                self.check_overflow(result)
                    .map_err(|kind| CalculatorError::new(kind, None))
            }
            _ => CalculatorError::new_err(NotValueReturn, None),
        }
    }

    pub fn calculate_line<'input>(
        &mut self,
        line: &'input str,
        index: Option<usize>,
    ) -> Result<Option<f64>, CalculatorError<'input>> {
        let wrap_err = |kind: CalculatorErrorKind<'input>| CalculatorError::new(kind, index);

        // it will call line_parser to parse
        let parse_result: Line = parse_line(&line)?;
        match parse_result {
            Line::Expression(expr) => {
                let result = self.handle_expression(&expr).map_err(wrap_err)?;
                self.check_overflow(result).map_err(wrap_err).map(Some)
            }
            Line::Sentence(id, expr) => {
                self.handle_sentence(&id, &expr).map_err(wrap_err)?;
                Ok(None)
            }
        }
    }

    // [Notice] reading and calculating must be decoupled
    // otherwise it will cause lifetime error
    pub fn calculate_file<'input>(
        &mut self,
        lines: Vec<&'input str>,
    ) -> Result<Vec<f64>, CalculatorError<'input>> {
        // init the status
        self.before_calculate();

        // using result array
        // to decouple computation and output
        let mut results: Vec<f64> = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if let Some(value) = self.calculate_line(line, Some(i))? {
                results.push(value)
            }
        }

        Ok(results)
    }

    pub fn clear(&mut self) {
        self.symbol_table.clear();
    }

    /* --------------- handler --------------- */
    pub(crate) fn handle_expression<'input>(
        &mut self,
        expr: &Expr,
    ) -> Result<f64, CalculatorErrorKind<'input>> {
        match expr {
            Expr::Id(id) => match id {
                ID::E => Ok(f64::consts::E),
                ID::Pi => Ok(f64::consts::PI),
                _ => {
                    if let Some(value) = self.symbol_table.get(id) {
                        Ok(*value)
                    } else {
                        Err(UndefinedIdError(*id))
                    }
                }
            },
            // when meeting inf or nan, return error
            Expr::Value(value) => {
                if value.is_infinite() || value.is_nan() {
                    Err(OverflowError)
                } else {
                    Ok(*value)
                }
            }
            Expr::Operation { l, r, opt } => self.handle_operation(l, r, opt),
            Expr::UnaryOperation { operand, opt } => self.handle_unary_operation(operand, opt),
        }
    }

    #[inline]
    fn handle_unary_operation<'input>(
        &mut self,
        operand: &Expr,
        opt: &UnaryOperator,
    ) -> Result<f64, CalculatorErrorKind<'input>> {
        // Avoid floating point errors introduced by rounding
        let rounding = |num: f64| {
            if (num - num.round()).abs() < 1e-10 {
                num.round()
            } else {
                num
            }
        };

        let operand_value = self.handle_expression(operand)?;
        match opt {
            // NOTE optimize: float can be factorial by gamma function
            UnaryOperator::Minus => Ok(-operand_value),
            UnaryOperator::Factorial => Ok(factorial::factorial(operand_value as u64)),
            UnaryOperator::Sin => Ok(rounding(operand_value.sin())),
            UnaryOperator::Cos => Ok(rounding(operand_value.cos())),
            UnaryOperator::Tan => Ok(rounding(operand_value.tan())),
        }
    }

    #[inline]
    fn handle_operation<'input>(
        &mut self,
        l: &Expr,
        r: &Expr,
        opt: &Operator,
    ) -> Result<f64, CalculatorErrorKind<'input>> {
        let left = self.handle_expression(l)?;
        let right = self.handle_expression(r)?;

        match opt {
            Operator::Plus => Ok(left + right),
            Operator::Sub => Ok(left - right),
            Operator::Mul => Ok(left * right),
            Operator::Div => {
                if right == 0.0 {
                    Err(ArithmeticError("Division by zero"))
                } else {
                    Ok(left / right)
                }
            }
            Operator::Power => Ok(left.powf(right)),
            Operator::Root => {
                if right == 0.0 {
                    Err(ArithmeticError("Root with zero index"))
                } else if right < 0.0 {
                    Err(ArithmeticError("Root with negative index"))
                } else if left < 0.0 && right % 2.0 == 0.0 {
                    Err(ArithmeticError("Odd root of a negative number"))
                } else {
                    Ok(right.powf(left.recip()))
                }
            }
            Operator::Log => {
                if left <= 0.0 || right <= 0.0 {
                    return Err(ArithmeticError("Log with zero base or zero argument"));
                }
                // NOTE can't use match on float type!
                // NOTE use log2 ,ln and log10 to get more precise result, maybe use other crates future
                Ok(if left == 2.0 {
                    right.log2()
                } else if left == f64::consts::E {
                    right.ln()
                } else if left == 10.0 {
                    right.log10()
                } else {
                    right.log(left)
                })
            }
        }
    }

    fn check_overflow<'input>(&mut self, result: f64) -> Result<f64, CalculatorErrorKind<'input>> {
        if result.is_infinite()
            || result.is_nan()
            || result
                .to_string()
                .chars()
                .filter(|&c| c.is_digit(10))
                .count()
                > 30
        {
            Err(OverflowError)
        } else {
            Ok(result)
        }
    }

    pub(crate) fn handle_sentence<'input>(
        &mut self,
        id: &ID,
        expr: &Expr,
    ) -> Result<(), CalculatorErrorKind<'input>> {
        let id_value = self.handle_expression(expr)?;
        self.symbol_table.insert(*id, id_value);
        Ok(())
    }
}

/* --------------- parser --------------- */

lalrpop_mod!(pub parser);

// encapsulate the lalrpop interface
fn parse_line(line: &str) -> Result<Line, CalculatorError> {
    match parser::LineParser::new().parse(line) {
        Ok(r) => Ok(r),
        Err(e) => CalculatorError::new_err(ParseError(e), None),
    }
}
