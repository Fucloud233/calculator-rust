use crate::calculator::Calculator;
use colored::*;
use std::fs;
use std::io::{self, Write};

pub fn interactive_mode() {
    println!(
        "{}{}",
        "Entering interactive mode. Type 'exit' or 'e' to quit.\n".cyan(),
        "Sentences, expressions, and file commands ('load <file_path>') are acceptable.\n".cyan()
    );
    let mut buffer = String::new();
    let mut calculator = Calculator::new();
    print!("{}", "> ".green());
    loop {
        io::stdout().flush().unwrap();
        buffer.clear();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let input = buffer.trim().to_string();
                if input == "exit" || input == "e" {
                    println!("{}", "> Exiting interactive mode.".green());
                    break;
                } else if input.starts_with("load ") {
                    let file_path = &input[5..];
                    file_interactive_mode(file_path, &mut calculator);
                    calculator.before_calculate();
                } else {
                    match calculator.handle_line(input.as_str(), true) {
                        Ok(Some(result)) => {
                            println!("{} {}", "> Result => :".green(), result.to_string().bold());
                            print!("{}", "> ".green());
                        }
                        Ok(None) => {
                            print!(
                                "{}{}{}",
                                "> ".green(),
                                "Statement executed successfully, no value returned.\n",
                                "> ".green()
                            );
                        }
                        Err(e) => {
                            println!("{} {}", "> An error occurred => :".red(), e);
                            print!("{}", "> ".red());
                        }
                    }
                }
            }
            Err(error) => {
                println!("{}{}", "> Error: ".red().bold(), error);
                break;
            }
        }
    }
}

pub fn file_interactive_mode(file_path: &str, calculator: &mut Calculator) {
    match fs::read_to_string(file_path) {
        Err(e) => {
            println!("{} {}", "=> Error reading file:".red(), e);
            print!("{}", "> ".red());
            return;
        }
        Ok(contents) => {
            println!("{}", "File interactive mode.".yellow());
            println!("{}", "Use 'run' or 'r' to run whole file , 'step' or 's' to move to next line ,  'exit' or 'e' to exit".yellow());
            let mut lines = contents.lines().collect::<Vec<&str>>();
            let mut line_index = 0;
            println!("{}\n{}","File contents:".cyan(), contents.yellow());
            print!("{}", "=> ".cyan());
            loop {
                io::stdout().flush().unwrap();
                let mut command = String::new();
                if io::stdin().read_line(&mut command).is_err() {
                    println!("{}", "Error reading command.".red());
                    continue;
                }
                match command.trim() {
                    "run" | "r" => {
                        for line in &lines[line_index..] {
                            execute_line(line, calculator);
                        }
                        print!(
                            "{}{}",
                            "=> Exiting file intercative mode, Clear all data => \n".cyan(),
                            "> ".green()
                        );
                        return;
                    }
                    "step" | "s" => {
                        if line_index < lines.len() {
                            execute_line(lines[line_index], calculator);
                            line_index += 1;
                            print!("{}", "=> ".cyan());
                        } else {
                            println!("{}", "=> No more lines to execute. exit file mod".cyan());
                            print!(
                                "{}{}",
                                "=> Exiting file intercative mode, Clear all data => \n".cyan(),
                                "> ".green()
                            );
                            return;
                        }
                    }
                    "exit" | "e" => {
                        print!(
                            "{}{}",
                            "=> Exiting file intercative mode, Clear all data => \n".cyan(),
                            "> ".green()
                        );
                        return;
                    }
                    _ => print!(
                        "{}",
                        "Unknown command. Use 'run', 'step', or 'exit'.\n> ".red()
                    ),
                }
            }
        }
    }
}

pub fn execute_line(line: &str, calculator: &mut Calculator) {
    match calculator.handle_line(line, true) {
        Ok(Some(result)) => {
            println!(
                "{} {}",
                "Line result => :".green(),
                result.to_string().bold()
            );
        }
        Ok(None) => {
            println!(
                "{}",
                "Line executed successfully, no value returned.".green()
            );
        }
        Err(e) => {
            println!("{} {}", "Line error => :".red(), e);
        }
    }
}
