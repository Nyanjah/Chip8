mod cpu;
mod render;
use cpu::{CHIP8, start_clock, run};
extern crate log;

// CHIP-8 Emulator written in the RUST programming language. 🦀 Praise Be Ferris 🦀
// by Jason Zingaretti

// Technical References:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fetch

fn main(){
    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Trace);
    let chip = CHIP8::new();
    start_clock(&chip);
    chip.lock().unwrap().load_program(String::from("./ROMS/Test.ch8"));
    run(&chip);
}