mod ast;

#[cfg(test)]
mod test {
    mod utils;
    mod lexer;
    mod level1;
    mod expr;
    mod factor;
}

fn main() {
    println!("Hello, world!");
}
