mod cpu;
mod render;
use cpu::CHIP8;
extern crate log;

// CHIP-8 Emulator written in the RUST programming language. 🦀 Praise Be Ferris 🦀
// by Jason Zingaretti

// Technical References:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fetch

fn main(){
    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Trace);
    let mut chip = CHIP8::new();
    chip.load_program(String::from("./ROMS/cavern.ch8"));
    chip.run();

}