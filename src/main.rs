use std::fs;
use text_io::read;

mod lexer;
mod token;
mod transform;

fn main() {
    println!("Enter your markdown (CTRL-D + ENTER to quit)");
    let s: String = read!("{}\u{0004}");
    let tokens = lexer::lex(s);

    let html = transform::transform(tokens);
    let template =
        fs::read_to_string("assets/template.html").expect("Something went wrong reading the file");

    // template.replace("{html}", {The html we make})
}
