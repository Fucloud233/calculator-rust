use colored::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "calc")]
struct Opt {
    // 你的其他StructOpt代码
}

fn main() {
    // let opt = Opt::from_args();
    // 你的程序逻辑

    println!("{}", "This is green text".green());
    println!("{}", "This is blue text on yellow background".blue().on_yellow());
    println!("{}", "This is bold text".bold());
}