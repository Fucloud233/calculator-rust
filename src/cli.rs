use crate::calculator::Calculator;
use crate::interactive::{interactive_mode, file_interactive_mode};
use colored::*;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
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
    match opt {
        Opt {
            expression: Some(expr),
            ..
        } => {
            let mut calculator = Calculator::new();
            match calculator.handle_line(&expr, false) {
                Ok(Some(result)) => {
                    println!("{} {}", "Result => :".green(), result.to_string().bold());
                }
                Ok(None) => {
                    println!(
                        "{}",
                        "Statement executed successfully, but no value returned.".green()
                    );
                }
                Err(e) => {
                    println!("{} {}", "An error occurred => :".red(), e);
                }
            }
        }
        Opt {
            path: Some(file_path),
            ..
        } => {
            println!("Reading file at: {}", file_path.display().to_string().cyan());
            if let Ok(contents) = fs::read_to_string(&file_path) {
                println!("File contents:\n{}", contents.yellow());
                let lines: Vec<&str> = contents.lines().collect();
                let mut calculator = Calculator::new();
            
                match calculator.calculate_file(lines) {
                    Ok(results) => {
                        for result in results {
                            println!("{} {}", "Result => :".green(), result.to_string().bold());
                        }
                    }
                    Err(e) => {
                        println!("{} {}", "Error => :".red(), e);
                    }
                }
            } else {
                println!("{}", "Error reading file".red());
            }
        }
        Opt {
            interactive: true, ..
        } => {
            interactive_mode();
        }
        _ => {
            match env::current_dir() {
                Ok(path) => {
                    println!("{} {}", "Current directory:".blue(), path.display().to_string().blue());
                }
                Err(e) => {
                    eprintln!("Error getting current directory: {}", e.to_string().red());
                    return;
                }
            }
            println!(
                "{}",
                "Usage: calc -e <expression> | -p <path> | -i | -h".bright_blue()
            );
        }
    }
}
