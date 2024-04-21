# CHIP-8 Emulator in Rust

This repo contains a CHIP-8 emulator implemented in Rust, a project I started to get into emulating low level hardware and it seemed like a great excuse to learn the Rust programming language.

I'd reccomend anyone who want's to get into emulation to give it a shot because It was really fun to work on and I learned a lot about how the CPU operates in older hardware.

![image](https://github.com/Nyanjah/Chip8/assets/65467278/32742f5a-6215-4a97-a93f-1fde69d6583c)

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)

## Introduction
The CHIP-8 emulator here was one of my first projects in Rust. It was a fun way to explore simulating low-level hardware and learning the Rust programming languge.
It provides a fully functional emulation of the CHIP-8 system, which is an interpreted programming language created in the mid 1970s which ran on older hardware like the COSMAC VIP.

The program is designed to emulate the CPU and I/O of the CHIP-8 system, including graphics rendering and sound integration, utilizing modern Rust practices and libraries.
It runs in the terminal and includes logging for each instruction being executed on the CPU as wella as a display to see the ROM being played.
Windows, Mac, and Linux are all supported.

## Features
- Full emulation of the CHIP-8 instruction set
- Real-time graphical output in terminal using `tui`
- Sound support through `rodio`
- Keyboard input handling for interactive emulation

## Installation
To get started with this emulator, clone the repository and build the project using Cargo, the Rust package manager:

```bash
git clone https://github.com/your-username/chip8-rust.git
cd chip8-rust
cargo build --release
```
## Usage
To use this emulator once installed, first import CPU:
use cpu::{CHIP8, start_clock, run};

## References

The development of this project is based on several resources that provide detailed information about CHIP-8. Below are key references used:

- [CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap) - This resource offers an extensive technical reference on the CHIP-8, including memory mapping and operational details.





