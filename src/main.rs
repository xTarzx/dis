use std::{collections::VecDeque, process::ExitCode, time::Duration};

type Result<T> = std::result::Result<T, ()>;

mod dis;
mod lexer;
mod statement;

use dis::DIS;

extern crate sdl2;

// use sdl2::{event::Event, keyboard::Keycode};
const MEM_OFFSET: usize = 1024;

// fn run_with_video(dis: &mut DIS) {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     let pixel_size: usize = 50;
//     let pixel_count: usize = 16;

//     let window = video_subsystem
//         .window(
//             "DIS",
//             (pixel_count * pixel_size) as u32,
//             (pixel_count * pixel_size) as u32,
//         )
//         .position_centered()
//         .build()
//         .unwrap();

//     let mut canvas = window.into_canvas().build().unwrap();

//     let mut event_pump = sdl_context.event_pump().unwrap();

//     'run: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyUp {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'run,

//                 _ => {}
//             }
//         }

//         dis.step();

//         for y in 0..pixel_count {
//             for x in 0..pixel_count {
//                 let pixel = dis.memory[MEM_OFFSET + y * pixel_count + x];

//                 let color = match pixel {
//                     0 => sdl2::pixels::Color::RGB(0, 0, 0),
//                     1 => sdl2::pixels::Color::RGB(255, 255, 255),
//                     _ => sdl2::pixels::Color::RGB(255, 0, 0),
//                 };

//                 canvas.set_draw_color(color);
//                 canvas
//                     .fill_rect(sdl2::rect::Rect::new(
//                         (x * pixel_size) as i32,
//                         (y * pixel_size) as i32,
//                         pixel_size as u32,
//                         pixel_size as u32,
//                     ))
//                     .unwrap();
//             }
//         }

//         canvas.present();

//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
//     }
// }

fn main() -> ExitCode {
    let mut video_support = false;
    let mut filepath: Option<String> = None;
    let mut args: VecDeque<String> = std::env::args().collect();

    let program = args.pop_front().unwrap();

    if args.len() < 1 {
        println!("Usage: {} <program.dis>", program);
        return ExitCode::FAILURE;
    }

    for arg in args.iter() {
        if arg.starts_with("-") {
            match &arg[1..] {
                "video" => {
                    eprintln!("Video support is not supported yet");
                    return ExitCode::FAILURE;

                    video_support = true
                }
                _ => {
                    eprintln!("Unknown argument: {}", arg);
                    return ExitCode::FAILURE;
                }
            }

            continue;
        }

        filepath = Some(arg.to_owned());
    }

    if filepath.is_none() {
        eprintln!("Usage: {} <program.dis>", program);
        return ExitCode::FAILURE;
    }

    let filepath = filepath.unwrap();

    let mut dis = DIS::new();

    if dis.load(filepath).is_err() {
        println!("Error loading program");
        return ExitCode::FAILURE;
    }

    // if video_support {
    //     run_with_video(&mut dis);
    //     return;
    // }

    dis.run();

    ExitCode::SUCCESS
}
