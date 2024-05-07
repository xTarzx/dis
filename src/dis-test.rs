mod dis;
mod lexer;
mod statement;

use dis::DIS;

type Result<T> = std::result::Result<T, ()>;

fn main() {
    let mut dis = DIS::new();
    match dis.load("examples/hello.dis") {
        Ok(_) => {
            println!("Loaded program successfully");
        }
        Err(_) => {
            eprintln!("Failed to load program");
        }
    };
}
