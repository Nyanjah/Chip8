# CHIP-8 Emulator in Rust

This repository contains a CHIP-8 emulator implemented in Rust, demonstrating proficiency in low-level system programming and embedded systems. 
The emulator is designed to offer a comprehensive understanding of the CHIP-8 system, including graphics rendering and sound integration, utilizing modern Rust practices and libraries.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)

## Introduction
The CHIP-8 emulator here was one of my first projects in Rust. It was a fun way to explore simulating low-level hardware and learning the Rust programming languge.
It provides a fully functional emulation of the CHIP-8 system, which is historically significant in the evolution of computer hardware and game development.

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





