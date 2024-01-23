use std::collections::VecDeque;

use dis::DIS;

fn main() {
    // get file from arg

    let mut args: VecDeque<String> = std::env::args().collect();

    let program = args.pop_front().unwrap();

    if args.len() < 1 {
        println!("Usage: {} <program.dis>", program);
        return;
    }

    let filepath = args.pop_front().unwrap();

    let mut dis = DIS::new();

    if dis.load_program(&filepath.as_str()).is_ok() {
        dis.run();
    };
}
