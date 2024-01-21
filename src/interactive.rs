use crate::calculator::Calculator;
use crate::utils::io::read_lines;
use colored::*;
use std::io::{self, Write};
use std::path::Path;

pub fn interactive_mode() {
    println!(
        "{}{}{}",
        "Entering interactive mode. Type 'exit' or 'e' to quit.\nYou can use  \\alpha, \\beta, x, y, z, a, b, c, d  as variables
        e is a constant,you can't change its value
        Sentences, expressions, and file commands ('".cyan(),"load <file_path>".bright_blue(),"') are acceptable.".cyan()
    );

    let mut buffer = String::new();
    let mut calculator = Calculator::new();
    print!("{}", "=> ".cyan());
    loop {
        io::stdout().flush().unwrap();
        buffer.clear();

        // check input
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(error) => {
                println!("{}{}", "Error: ".red().bold(), error);
                break;
            }
        }

        let input = buffer.trim().to_string();
        if input == "exit" || input == "e" {
            println!("{}", "Exiting interactive mode.".green());
            break;
        } else if input.starts_with("load ") {
            let file_path = &input[5..];
            file_interactive_mode(file_path, &mut calculator);
            calculator.clear();
            continue;
        }

        execute_line(&input,&mut calculator);
        print!("{}", "=> ".cyan());
    }
}

pub fn file_interactive_mode(file_path: &str, calculator: &mut Calculator) {
    if Path::new(file_path).extension().unwrap_or_default() != "calc" {
        print!(
            "{}{}",
            "=> Error: Only .calc files are supported\n".red(),
            "=> ".red()
        );
        return;
    }
    match read_lines(file_path) {
        Err(e) => {
            println!("{} {}", "=> Error reading file:".red(), e);
            print!("{}", "=> ".red());
            return;
        }
        Ok(lines) => {
            println!("{}", "File interactive mode.".yellow());
            println!("{}", "Use 'run' or 'r' to run whole file, 'step' or 's' to move to next line, 'exit' or 'e' to exit".yellow());
            let mut line_index = 0;
            println!("{}\n{}", "File contents:".cyan(), lines.join("\n").yellow());
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
                            "=> Exiting file interactive mode, Clear all data => \n".cyan(),
                            "=> ".green()
                        );
                        return;
                    }
                    "step" | "s" => {
                        if line_index < lines.len() {
                            execute_line(&lines[line_index], calculator);
                            line_index += 1;
                            print!("{}", "=> ".cyan());
                        } else {
                            println!("{}", "=> No more lines to execute. Exit file mode.".cyan());
                            print!(
                                "{}{}",
                                "=> Exiting file interactive mode, Clear all data => \n".cyan(),
                                "=> ".green()
                            );
                            return;
                        }
                    }
                    "exit" | "e" => {
                        print!(
                            "{}{}",
                            "=> Exiting file interactive mode, Clear all data => \n".cyan(),
                            "=> ".green()
                        );
                        return;
                    }
                    _ => print!(
                        "{}",
                        "Unknown command. Use 'run', 'step', or 'exit'.\n> ".red(),
                    ),
                }
            }
        }
    }
}

pub fn execute_line(line: &str, calculator: &mut Calculator) {
    // 获取终端宽度以用于格式化输出
    let terminal_width = term_size::dimensions().map_or(80, |(w, _)| w);

    let line_display_width = 30;
    let line_trimmed = if line.len() > line_display_width {
        format!("{}...", &line[..line_display_width - 3])
    } else {
        line.to_string()
    };

    let result = match calculator.calculate_line(line, None) {
        Ok(Some(result)) => format!("Line result => : {}", result).green(),
        Ok(None) => "Line executed successfully, no value returned.".to_string().green(),
        Err(e) => format!("Line error => : {}", e.message()).red(),
    };

    let formatted_output = format!(
        "{:<width$}| Expression: {}",
        "",
        line_trimmed.cyan(),
        width = terminal_width - line_display_width
    );

    println!("{}\n{}",result, formatted_output);
}
