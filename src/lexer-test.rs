mod lexer;
use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("examples/fizzbuzz.dis".to_string());

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}
