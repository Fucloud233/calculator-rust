use cli::run_cli;

mod ast;
mod cli;
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
mod interactive;

mod utils {
    pub mod error;
    pub mod file;
}

fn main() {
    run_cli();
}

