mod dis;
mod lexer;

use dis::DIS;

fn main() {
    let mut dis = DIS::new();
    match dis.load("examples/simple.dis") {
        Ok(_) => {
            println!("Loaded program:");
        }
        Err(_) => {
            eprintln!("Failed to load program");
        }
    };
}
