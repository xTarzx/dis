use std::{collections::VecDeque, process::ExitCode};

mod dis;
mod lexer;
mod statement;

use dis::DIS;

fn main() -> ExitCode {
    let mut args: VecDeque<String> = std::env::args().collect();

    let program = args.pop_front().unwrap();

    if args.len() < 1 {
        println!("Usage: {} <program.dis>", program);
        return ExitCode::FAILURE;
    }

    let filepath = args.pop_front().unwrap();

    let mut dis = DIS::new();

    if dis.load(filepath).is_err() {
        println!("Error loading program");
        return ExitCode::FAILURE;
    }

    dis.run();

    ExitCode::SUCCESS
}
