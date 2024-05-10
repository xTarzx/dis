use std::fmt;

use raylib::prelude::*;

use itertools::Itertools;

use dis::lexer::Token;
use dis::statement::{Op, Statement};
use dis::DIS;

fn format_statement(statement: &Statement) -> String {
    let mut s = String::new();

    if let Some(label) = &statement.label {
        let label = match label {
            Token::Label { value, .. } => value,
            _ => unreachable!(),
        };

        s.push_str(label);
        s.push_str(": ");
    }

    let op = match statement.op {
        Op::MOV(_) => "mov",
        Op::ADD(_) => "add",
        Op::SUB(_) => "sub",
        Op::CMP(_) => "cmp",
        Op::JLT(_) => "jlt",
        Op::JGT(_) => "jgt",
        Op::JEQ(_) => "jeq",
        Op::JNE(_) => "jne",
        Op::JMP(_) => "jmp",
        Op::RUN(_) => "run",
        Op::RET(_) => "ret",
        Op::DIE(_) => "die",
        Op::OUT(_) => "out",
        Op::PRT(_) => "prt",
        Op::DBG(_) => "dbg",
        Op::INC(_) => "@",
        Op::RDN(_) => "rdn",
        Op::RDC(_) => "rdc",
        Op::RLN(_) => "rln",
        Op::NOP => "nop",
    };

    s.push_str(op);

    for arg in &statement.body {
        s.push_str(format!(" {}", arg).as_str());
    }

    s
}

fn draw_registers(d: &mut RaylibDrawHandle, dis: &DIS) {
    let font_size = 24;
    let y_offset = 32 + font_size * 2;

    d.draw_text("REG", 0, y_offset - font_size, font_size, Color::WHITE);

    for (idx, reg_id) in dis.registers.keys().sorted().enumerate() {
        let x = font_size * (idx as i32 % 4) * 4;
        let y = font_size * (idx as i32 / 4) * 2 + 32;

        let s = format!("{:>2}: {:04x}", reg_id, dis.registers[reg_id]);
        d.draw_text(&s, x, y + y_offset, font_size, Color::WHITE);
    }
}
#[derive(Clone, Copy)]
enum MemMode {
    HEX,
    CHAR,
}

impl fmt::Display for MemMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemMode::HEX => write!(f, "HEX"),
            MemMode::CHAR => write!(f, "CHR"),
        }
    }
}

fn draw_memory(d: &mut RaylibDrawHandle, dis: &DIS, start: usize, step: usize, mode: MemMode) {
    let font_size = 24;
    let y_offset = 32 + font_size * 2;
    let x_offset = 32 * 13;

    let end = (start + step).min(dis.memory.len());

    d.draw_text(
        format!("MEM <{start}-{end}> : {mode}").as_str(),
        x_offset,
        y_offset - font_size,
        font_size,
        Color::WHITE,
    );
    for (idx, val) in dis.memory[start..end].iter().enumerate() {
        let x = font_size * (idx as i32 % 4) * 4 + x_offset;
        let y = font_size * (idx as i32 / 4) * 2 + 32;

        let s = match mode {
            MemMode::HEX => format!("{:04x}", val),
            MemMode::CHAR => {
                if *val >= 0x20 && *val <= 0x7e {
                    format!("{}", char::from_u32(*val as u32).unwrap_or('.'))
                } else {
                    format!("####")
                }
            }
        };
        d.draw_text(&s, x, y + y_offset, font_size, Color::WHITE);
    }
}

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    let mut dis = DIS::new();
    if dis.load("examples/hello.dis").is_err() {
        eprintln!("Error loading program");
        return;
    }

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("vis")
        .build();

    let bg_color = Color::get_color(0x333333ff);

    let mut mem_start = 0;
    let step = 32;

    let mut mode = MemMode::HEX;

    let mut buf = Vec::new();
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            dis.step(&mut buf);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            if mem_start > 0 {
                mem_start -= step;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            mem_start += step;

            if mem_start >= dis.memory.len() {
                mem_start = dis.memory.len() - step;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            mode = MemMode::CHAR;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            mode = MemMode::HEX;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        let cur_statement = &dis.program[dis.pc.min(dis.program.len() - 1)];
        d.draw_text(&format_statement(cur_statement), 0, 0, 32, Color::WHITE);

        draw_registers(&mut d, &dis);
        draw_memory(&mut d, &dis, mem_start, step, mode);

        {
            let font_size = 32;
            d.draw_text(
                buf.iter().map(|v| *v as char).join("").as_str(),
                0,
                WINDOW_HEIGHT - font_size,
                font_size,
                Color::WHITE,
            );
        }
    }
}
