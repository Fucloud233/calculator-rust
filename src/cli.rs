use crate::calculator::Calculator;
use crate::interactive::interactive_mode;
use crate::utils::error::CalculatorError;
use std::env;
use std::fs;
use std::path::PathBuf;
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Calculate an expression directly, only expression is acceptable
    #[structopt(short = "e", long = "expression")]
    expression: Option<String>,

    /// Path to a file with expressions
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: Option<PathBuf>,

    /// Enter interactive mode
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,
}

fn print_value(value: f64) {
    println!("{} {}", "Result => :".green(), value.to_string().bold())  
}

fn print_error(error: CalculatorError) {
    // TODO: perfect error display
    eprintln!("{} {}", "An error occurred => :".red(), error.message().bold())  
}

pub fn run_cli() {
    let opt = Opt::from_args();

    if let Some(expr) = opt.expression {
        let mut calculator = Calculator::new();
        match calculator.calculate_expr(&expr) {
            Ok(value) => print_value(value),
            Err(error) => print_error(error)
        }
    } else if let Some(file_path) = opt.path {
        println!("Reading file at: {}", file_path.display().to_string().cyan());
        let lines = match fs::read_to_string(&file_path) {
            Ok(contents) => {
                println!("File contents:\n{}", contents.yellow());
                contents.lines().collect::<Vec<&str>>()
            }, 
            Err(_) => {
                eprintln!("{}", "Error reading file".red());
                return
            }
        };
            
        match Calculator::new().calculate_file(lines) {
            Ok(values) => values.iter().for_each(|value| print_value(*value)),
            Err(error) => print_error(error)
        }
    } else if opt.interactive {
        interactive_mode();
    } else {
        match env::current_dir() {
            Ok(path) => 
                println!("{} {}", "Current directory:".blue(), path.display().to_string().blue()),
            Err(e) => {
                eprintln!("Error getting current directory: {}", e.to_string().red());
                return
            }
        }
        println!(
            "{}", "Usage: calc -e <expression> | -p <path> | -i | -h".bright_blue()
        );
    }
}
