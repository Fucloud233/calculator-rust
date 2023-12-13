mod ast;
use crate::calculator::Calculator;
use crate::utils::file::read_lines;
use colored::*;
use std::fs;
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
    /// Calculate an expression directly, only direct expression is acceptable
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
            // 处理单个表达式
            let mut calculator = Calculator::new();
            
            // 调用 calculate_expr 函数
            match calculator.calculate_expr(&expr) {
                Ok(result) => {
                    println!("{} {}", "Result => :".green(), result);
                }
                Err(e) => {
                    // 如果有错误，处理错误
                    println!("{} {:?}","An error occurred => :".red(), e);
                }
            }
        }
        Opt {
            path: Some(file_path),
            ..
        } => {
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
            println!("Usage: calc -e <expression> | -p <path> | -i");
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
        "{}",
        "Entering interactive mode. Type 'exit' to quit."
            .green()
    );
    // 这里应该是你的交互模式逻辑
    // 例如：读取用户输入，计算表达式，显示结果
    // 在这个简单的例子中，我们将直接退出
    println!("{}", "Exiting interactive mode.".green().bold());
}
