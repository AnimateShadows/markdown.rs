use text_io::read;
mod lexer;
mod token;

fn main() {
    println!("Enter your markdown (CTRL-D + ENTER to quit)");
    let s: String = read!("{}\u{0004}");
    lexer::lex(s);
    println!("Ended!");
}
