mod ast;

#[cfg(test)]
mod test {
    mod utils;
    mod lexer;
    mod level1;
    mod expr;
    mod factor;

    mod calculator {
        mod unit;
    }
}

mod calculator;

mod utils {
    pub mod file;
    pub mod error;
}

fn main() {
    println!("Hello, world!");
}
