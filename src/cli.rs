use crate::calculator::Calculator;
use crate::interactive::interactive_mode;
use crate::utils::io::{print_error, print_value};
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

pub fn run_cli() {
    let opt = Opt::from_args();

    if let Some(expr) = opt.expression {
        let mut calculator = Calculator::new();
        match calculator.calculate_expr(&expr) {
            Ok(value) => print!("{} {}", "Result => :".green(), value.to_string().bold()),
            Err(error) => eprint!("{} {}", "An error occurred => :".red(), error.message().bold())
        }
    } else if let Some(file_path) = opt.path {
        if file_path.extension().and_then(|s| s.to_str()) != Some("calc") {
            print!(
                "{}{}",
                "=> Error: Only .calc files are supported\n".red(),
                "existing cli".red()
            );
            return;
        }
        println!("Reading file at: {}", file_path.display().to_string().cyan());
        let lines = match fs::read_to_string(&file_path) {
            Ok(contents) => {
                println!("File contents:\n{}", contents.yellow());
                contents.lines().map(|line| line.into()).collect::<Vec<String>>()
            }, 
            Err(_) => {
                eprintln!("{}", "Error reading file".red());
                return
            }
        };

        let ref_lines = lines.iter().map(|line| line.as_str()).collect::<Vec<&str>>();
        match Calculator::new().calculate_file(ref_lines) {
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
