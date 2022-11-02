#![allow(dead_code)]
use crate::render::ChipRender;
use std::{thread::sleep, time::Duration};

// CHIP-8 Emulator written in the RUST programming language. 🦀 Praise Be Ferris 🦀
// by Jason Zingaretti

// Technical References:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fetch

// CHIP-8 SPECIFICATION DETAILS
pub struct CHIP8 {
    pub memory: [u8; 4096],         // 4KB ~ 4,096 Bytes RAM (0x00-0x200 reserved)
    pub pc: u16,                    // Program Counter (16-bit)
    pub index: u16,                 // Index Register  (16-bit)
    pub stack: Vec<u16>,            // Stack of (16-bit) addresses
    pub delay_timer: u8,            // Delay Timer 8-bit @ 60Hz
    pub sound_timer: u8,            // Sound Timer 8-bit @ 60Hz
    pub variables: [u8; 16],        // General purpose variable registers (0x0-0xF)
    pub display: [[bool; 32]; 64],  // Display output of 64 by 32 pixels

    pub renderer:ChipRender         // Renderer struct for the chip
}

impl CHIP8 {
    // Read from memory
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    // Write to memory
    fn mem_write(&mut self, addr: u16, data: u8) -> () {
        self.memory[addr as usize] = data
    }

    pub fn load_program(&mut self, path:String) -> () {
        // Taking in a vector of bytes from a file
        let program = std::fs::read(path).unwrap(); 
        // Copying the program data into memory starting from Byte 512 (0x200)
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program[..]);
        self.pc = 0x200; // Initializing program counter
    }

    // Initializing CHIP-8
    pub fn new() -> CHIP8 {
        let mut memory: [u8; 4096] = [0x00; 4096];

        // The address space [0x000 to 0x200) is reserved for the interpreter,
        // while [0x200 to 0xFFF] is free RAM for the programs to use.

        let font_data: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // Loading font data in the standard range of 0x050-0x09F
        memory[0x050..=0x09F].copy_from_slice(&font_data);

        CHIP8 {
            memory: memory,
            pc: 0x00,
            index: 0x00,
            stack: Vec::new(),
            delay_timer: 0x00,
            sound_timer: 0x00,
            variables: [0; 16],
            display: [[false; 32]; 64],
            renderer:ChipRender::setup().expect("Failed to setup renderer")
        }
    }

    pub fn run(&mut self) -> () {
        loop{
            sleep(Duration::from_millis(10));
            // FETCH STAGE
            sleep(std::time::Duration::from_millis(10));
            // Fetch the instruction from the program counter
            let inst_part1 = self.mem_read(self.pc);
            let inst_part2 = self.mem_read(self.pc + 1);
            // Combining both parts of the instruction to make the true instruction
            // using bit-wise or and padding with zeros.
            let instruction: u16 = ((inst_part1 as u16) << 8) | inst_part2 as u16;
            // Increment the program counter by 2
            self.pc = self.pc + 2;

            // DECODE STAGE

            // Extracting information from the instruction half-bytes (nibbles or nybbles, lol)
            // each nibble numbered (1-4) encodes information to be decoded before execution.
            let op   = (instruction & 0xF000) >> 12;  // Opcode, encodes which type of instruction
            let x    = (instruction & 0x0F00) >> 8;   // Used to look up a variable register (0x0-0xf)
            let y    = (instruction & 0x00F0) >> 4;   // Also used to look up a variable register (0x0-0xf)
            let n    = instruction & 0x000F;          // 4-bit  number
            let byte = instruction & 0x00FF;          // 8-bit  immediate number
            let addr = instruction & 0x0FFF;          // 12-bit immediate memory address

            // EXECUTE STAGE
            // This match statement contains all the instruction logic that can be executed by the CHIP-8,
            // implemented according to their original corresponding functionality.
            match op{   
                // Clear screen (00E0)
                0 => {
                    self.display = [[false; 32]; 64];
                    self.renderer.render(& mut self.display);
                }

                // Jump (1NNN)
                1 =>{
                    self.pc = addr
                }
                // Set register vx (6XNN)
                6 => {
                    self.variables[x as usize ] = byte as u8;

                }
                // Add value to register vx (7XNN)
                7 => {
                    self.variables[y as usize] += byte as u8;
                }
                // Set index register I (ANNN)
                0xA =>{
                    self.index = addr;

                }
                // Draw to screen  (DXYN)
                0xD =>{
                    // Getting the co-ordinates to be drawn to from registers vx and vy
                    let x_pos = ((self.variables[x as usize])% 64 ) as usize; // x mod 64
                    let y_pos = ((self.variables[y as usize])% 32 ) as usize; // y mod 32
                    let mut current_y_pos = y_pos;
                    
                    // Setting flag register to zero.
                    self.variables[15] = 0x0;
                    // For "n" rows on the screen
                    for row in 0..n{
                        let mut current_x_pos = x_pos;
                        // Getting a row of sprite data from the address stored in Index (I)
                        // This is the n'th sprite data byte corresponding to the n'th row
                        let row_of_sprite_data = self.memory[self.index as usize + row as usize];
                        // Iterating over the bits in the chosen sprite byte as boolean value "bit"
                        for bit in (0..8).rev().map(|i| (row_of_sprite_data >> i) & 1 == 1){
                            // If the bit on screen is on and the pixel of the sprite is on, turn
                            // off the pixel and set the flag register (vf) to 1.
                            if bit == true && self.display[current_x_pos][current_y_pos] == true{
                                self.display[current_x_pos][current_y_pos] = false;   // Turning off the pixel
                                self.variables[15] = 1;                               // Setting vf to 1
                            }
                            // Otherwise if the bit on the screen is off and the pixel sprite is on,
                            // turn on the pixel and set the flag register (vf) to 1.
                            else if bit == true && self.display[current_x_pos][current_y_pos] == false{
                                self.display[current_x_pos][current_y_pos] = true;    // Turning on the pixel
                            }
                            // If we reach the right edge of the screen, stop drawing the current row
                            if current_x_pos == 63{
                                current_x_pos = x_pos;
                                break
                            }
                            // Otherwise we continue and increment the x position of the current pixel on the display
                            current_x_pos = current_x_pos + 1;
                        }
                        // Stop if we reached the bottom of the screen
                        
                        if current_y_pos == 31{
                            break
                        }   
                        current_y_pos = current_y_pos + 1;
                    }
                    self.renderer.render(& mut self.display); // Render the sprite on screen
                }
                

                // Catch-all for unrecognized instructions
                _ =>{
                    panic!("Opcode {} not found",op);
                }
            }
        }
    }
}
