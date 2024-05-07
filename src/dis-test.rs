mod dis;
mod lexer;
mod statement;

use dis::DIS;

type Result<T> = std::result::Result<T, ()>;

fn main() {
    let mut dis = DIS::new();
    match dis.load("examples/mul.dis") {
        Ok(_) => {
            println!("Loaded program successfully");
            dis.run();
        }
        Err(_) => {
            eprintln!("Failed to load program");
        }
    };
}
