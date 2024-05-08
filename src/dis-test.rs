mod dis;
mod lexer;
mod statement;

use dis::DIS;

fn main() {
    let filepath = "examples/err/mislabel.dis";

    let mut dis = DIS::new();
    match dis.load(filepath) {
        Ok(_) => {
            println!("Loaded program successfully");
            dis.run();
        }
        Err(_) => {
            eprintln!("Failed to load program");
        }
    };
}
