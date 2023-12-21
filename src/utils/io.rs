use std::{fs::read_to_string, io};
use colored::Colorize;
use super::error::CalculatorError;

pub fn read_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let data = read_to_string(file_path)?;

    let lines = data.lines()
        .map(|text| text.to_string())
        .collect::<Vec<String>>();

    Ok(lines)
}


pub fn print_value(value: f64) {
    print!("{} {}\n{}", "Result => :".green(), value.to_string().bold(),"> ".green())  
    
}

pub fn print_error(error: CalculatorError) {
    // TODO: perfect error display
    eprint!("{} {}\n{}", "An error occurred => :".red(), error.message().bold(),"> ".red())  
}


// pub fn to_ref_lines(lines: &Vec<String>) -> &Vec<&'static str> {
//     lines.iter().map(|line| line.as_ref()).collect()
// }