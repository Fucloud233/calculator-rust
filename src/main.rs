mod ast;
use crate::calculator::Calculator;
use crate::utils::file::read_lines;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;
#[cfg(test)]
mod test {
    mod expr;
    mod factor;
    mod level1;
    mod lexer;
    mod utils;

    mod calculator {
        mod unit;
    }
}

mod calculator;

mod utils {
    pub mod error;
    pub mod file;
}

#[derive(StructOpt, Debug)]
#[structopt(name = "calc", about = "A simple calculator")]
struct Opt {
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

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt {
            expression: Some(expr),
            ..
        } => {
            let mut calculator = Calculator::new();
            match calculator.handle_line(&expr, false) {
                Ok(Some(result)) => {
                    // 处理有数值结果的情况
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
            // TODO:
            // 从文件中读取并处理表达式
            if let Ok(contents) = fs::read_to_string(file_path) {
                for line in contents.lines() {
                    let result = calculate_expression(line);
                    println!("Result: {}", result);
                }
            } else {
                eprintln!("Error reading file");
            }
        }
        Opt {
            interactive: true, ..
        } => {
            // 进入交互模式
            interactive_mode();
        }
        _ => {
            // 显示帮助信息或错误提示
            println!(
                "{}",
                "Usage: calc -e <expression> | -p <path> | -i".bright_blue()
            );
        }
    }
}

fn calculate_expression(expr: &str) -> f64 {
    // 这里应该是你的表达式计算逻辑
    // 作为示例，我们假设它总是返回固定值
    42.0
}

fn interactive_mode() {
    println!(
        "{},{}",
        "Entering interactive mode. Type 'exit' to quit. ==>\n".green(),
        "sentences and expressions are acceptable, input one line at a time\n".green()
    );
    // 创建一个用于读取输入的缓冲区
    let mut buffer = String::new();

    loop {
        // 打印提示符并刷新输出，以确保提示符立即显示
        print!("> ");
        io::stdout().flush().unwrap();

        let mut calculator = Calculator::new();
        // 读取用户输入
        buffer.clear();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                // 删除输入字符串末尾的换行符
                let input = buffer.trim().to_string();

                // 处理用户输入，如果输入特定命令（如"exit"）则退出
                if input == "exit" {
                    break;
                }

                // 在这里调用你的处理函数（例如计算表达式或其他命令）
                // 例如：handle_input(input);

                // 可以在此处输出结果或反馈
                // println!("Result: {}", result);
            }
            Err(error) => {
                println!("Error: {}", error);
                break;
            }
        }
    }

    // 这里应该是你的交互模式逻辑
    // 例如：读取用户输入，计算表达式，显示结果
    // 在这个简单的例子中，我们将直接退出
    println!("{}", "Exiting interactive mode.".green());
}
