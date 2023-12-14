use crate::calculator::Calculator;
use colored::*;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;

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

pub fn run_cli(){
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
            println!("{}", "Calculation finished :".green());
        }
        Opt {
            path: Some(file_path),
            ..
        } => {
            println!("Reading file at: {}", file_path.display().to_string().cyan());
            if let Ok(contents) = fs::read_to_string(file_path) {
                let lines: Vec<&str> = contents.lines().collect(); // 将文件内容转换为字符串切片向量
                let mut calculator = Calculator::new(); // 创建 Calculator 实例
            
                match calculator.calculate_file(lines) {
                    Ok(results) => {
                        // 打印每行的计算结果
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
            // 进入交互模式
            interactive_mode();
        }
        _ => {
            match env::current_dir() {
                Ok(path) => {
                    println!("{} {}","Current directory:".blue(), path.display().to_string().blue());
                }
                Err(e) => {
                    eprintln!("Error getting current directory: {}", e.to_string().red());
                    return; // 如果无法获取路径，则退出程序
                }
            }
            println!(
                "{}",
                "Usage: calc -e <expression> | -p <path> | -i | -h".bright_blue()
            );
            
        }
    }
}

fn interactive_mode() {
    println!(
        "{}{}",
        "Entering interactive mode. Type 'exit' or 'e' to quit. \n".green(),
        "sentences and expressions are acceptable, input one line at a time ==>".green()
    );
    // 创建一个用于读取输入的缓冲区
    let mut buffer = String::new();
    let mut calculator = Calculator::new();
    print!("{}","> ".green());
    
    loop {
        io::stdout().flush().unwrap();
        
        // 读取用户输入
        buffer.clear();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                // 删除输入字符串末尾的换行符
                let input = buffer.trim().to_string();
                if input == "exit"||input == "e" {
                    println!("{}", "> Exiting interactive mode.".green());
                    break;
                }else{
                    match calculator.handle_line(input.as_str(), true) {
                        Ok(Some(result)) =>{
                            println!("{} {}", "> Result => :".green(), result.to_string().bold());
                            print!("{}","> ".green());
                        }
                        Ok(None) =>{
                            print!(
                                "{}{}{}",
                                "> ".green(),
                                "Statement executed successfully, no value returned.\n",
                                "> ".green()
                            );
                        }
                        Err(e) =>{
                            println!("{} {}", "> An error occurred => :".red(), e);
                            print!("{}","> ".red());
                        }
                    }
                }
            
            }
            Err(error) => {
                println!("{}{}", "> Error: ".red().bold(),error);
                break;
            }
        }
    }
}
