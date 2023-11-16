mod ast;

#[cfg(test)]
mod test {
    mod utils;
    mod lexer;
    mod level1;
    mod expr;
    mod factor;
}

mod calculator;

mod utils {
    pub mod file;
}

fn main() {
    println!("Hello, world!");
}
