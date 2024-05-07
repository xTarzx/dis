mod dis;
mod lexer;

use dis::DIS;

fn main() {
    let mut dis = DIS::new();
    dis.load("examples/simple.dis");
}
