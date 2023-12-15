use std::io::{self, Write};
use colored::*;
use crate::calculator::Calculator;
use crate::utils::io::{print_error, print_value};

pub fn interactive_mode() {
    println!(
        "{}",
        "Entering interactive mode. Type 'exit' or 'e' to quit.\nYou can use a , b , c as variables\n, \
        Sentences, expressions, and file commands ('load <file_path>') are acceptable.\n".cyan()
    );

    let mut buffer = String::new();
    let mut calculator = Calculator::new();

    loop {
        eprint!("{}", "> ".green());
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
            // file_interactive_mode(file_path, &mut calculator);
            calculator.clear();
            continue;
        } 

        match calculator.calculate_line(&input, None) {
            Ok(Some(value)) => print_value(value),
            Err(e) => print_error(e),
            Ok(None) => println!("Statement executed successfully, no value returned.")
        }
        
    }
}

// pub fn file_interactive_mode(file_path: &str, calculator: &mut Calculator) {
//     if Path::new(file_path).extension().unwrap_or_default() != "calc" {
//         println!("{} {}", "=> Error: Only .calc files are supported".red(), "> ".red());
//         return;
//     }

//     match fs::read_to_string(file_path) {
//         Err(e) => {
//             println!("{} {}", "=> Error reading file:".red(), e);
//             print!("{}", "> ".red());
//             return;
//         }
//         Ok(contents) => {
//             println!("{}", "File interactive mode.".yellow());
//             println!("{}", "Use 'run' or 'r' to run whole file , 'step' or 's' to move to next line ,  'exit' or 'e' to exit".yellow());
//             let mut lines = contents.lines().collect::<Vec<&str>>();
//             let mut line_index = 0;
//             println!("{}\n{}", "File contents:".cyan(), contents.yellow());
//             print!("{}", "=> ".cyan());
//             loop {
//                 io::stdout().flush().unwrap();
//                 let mut command = String::new();
//                 if io::stdin().read_line(&mut command).is_err() {
//                     println!("{}", "Error reading command.".red());
//                     continue;
//                 }

//                 match command.trim() {
//                     "run" | "r" => {
//                         for line in &lines[line_index..] {
//                             execute_line(line, calculator);
//                         }
//                         print!(
//                             "{}{}",
//                             "=> Exiting file intercative mode, Clear all data => \n".cyan(),
//                             "> ".green()
//                         );
//                         return;
//                     }
//                     "step" | "s" => {
//                         if line_index < lines.len() {
//                             execute_line(lines[line_index], calculator);
//                             line_index += 1;
//                             print!("{}", "=> ".cyan());
//                         } else {
//                             println!("{}", "=> No more lines to execute. exit file mod".cyan());
//                             print!(
//                                 "{}{}",
//                                 "=> Exiting file intercative mode, Clear all data => \n".cyan(),
//                                 "> ".green()
//                             );
//                             return;
//                         }
//                     }
//                     "exit" | "e" => {
//                         print!(
//                             "{}{}",
//                             "=> Exiting file intercative mode, Clear all data => \n".cyan(),
//                             "> ".green()
//                         );
//                         return;
//                     }
//                     _ => print!(
//                         "{}",
//                         "Unknown command. Use 'run', 'step', or 'exit'.\n> ".red()
//                     ),
//                 }
//             }
//         }
//     }
// }

// pub fn execute_line(line: &str, calculator: &mut Calculator) {
//     // 获取终端宽度以用于格式化输出
//     let terminal_width = match term_size::dimensions() {
//         Some((w, _)) => w,
//         None => 80,  // 默认终端宽度
//     };

//     let line_display_width = 30;  // 分配给表达式显示的宽度
//     let line_trimmed = if line.len() > line_display_width {
//         &line[..line_display_width - 3]  // 如果太长，则截断并加上省略号
//     } else {
//         line
//     };

//     // match calculator.handle_line(line, true) {
//     //     Ok(Some(result)) => {
//     //         let result_str = format!("{:<width$}", format!("Line result => : {}", result).green(), width = terminal_width - line_display_width);
//     //         println!("{}{}", result_str, format!("| Expression: {}", line_trimmed).cyan());
//     //     }
//     //     Ok(None) => {
//     //         let result_str = format!("{:<width$}", "Line executed successfully, no value returned.".green(), width = terminal_width - line_display_width);
//     //         println!("{}{}", result_str, format!("| Expression: {}", line_trimmed).cyan());
//     //     }
//     //     Err(e) => {
//     //         let result_str = format!("{:<width$}", format!("Line error => : {}", e).red(), width = terminal_width - line_display_width);
//     //         println!("{}{}", result_str, format!("| Expression: {}", line_trimmed).cyan());
//     //     }
//     // }
// }
