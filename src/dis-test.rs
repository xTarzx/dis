mod dis;
mod lexer;
mod statement;

use dis::DIS;

fn main() {
    let mut dis = DIS::new();
    match dis.load("examples/debug.dis") {
        Ok(_) => {
            println!("Loaded program successfully");
            dis.run();
        }
        Err(_) => {
            eprintln!("Failed to load program");
        }
    };
}
