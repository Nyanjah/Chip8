#![allow(dead_code)]
use std::sync::{Mutex, Arc};
use std::time::Duration;
use crate::render::ChipRender;
use device_query::Keycode;
use device_query::{DeviceQuery, DeviceState, Keycode::*};
use rand::Rng;
use rodio::source::SineWave;
use spin_sleep;
use rodio::{self, Source, OutputStream};

// CHIP-8 Emulator written in the RUST programming language. 🦀 Praise Be Ferris 🦀
// by Jason Zingaretti

// Technical References:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fetch

pub fn start_clock(chip8:&Arc<Mutex<CHIP8>>) -> (){
    let chip8 = Arc::clone(chip8);
    std::thread::spawn( move ||{
        // Playing audio using sound timer
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let source = SineWave::new(43.648).take_duration(Duration::from_secs_f32(1.0));
            loop {
            spin_sleep::sleep(Duration::from_secs_f32(1.0/60.0));
            let mut chip8 = chip8.lock().unwrap();
            if chip8.sound_timer != 0{
                stream_handle.play_raw(source.clone()).unwrap();      
            }
            chip8.delay_timer = chip8.delay_timer.saturating_sub(1);
            chip8.sound_timer = chip8.sound_timer.saturating_sub(1);
            log::error!("timer: {}",chip8.delay_timer);
            drop(chip8);
        }

    });
}


// CHIP-8 SPECIFICATION DETAILS
pub struct CHIP8 {
    pub memory: [u8; 4096],            // 4KB ~ 4,096 Bytes RAM (0x00-0x200 reserved)
    pub pc: u16,                       // Program Counter (16-bit)
    pub index: u16,                    // Index Register  (16-bit)
    pub stack: Vec<u16>,               // Stack of (16-bit) addresses
    pub delay_timer: u8,               // Delay Timer 8-bit @ 60Hz
    pub sound_timer: u8,               // Sound Timer 8-bit @ 60Hz
    pub variables: [u8; 16],           // General purpose variable registers (0x0-0xF)
    pub display: [[bool; 32]; 64],     // Display output of 64 by 32 pixels

    pub renderer: ChipRender, // Renderer struct for the chip
    pub config: Config,       // Configurable chip-8 settings
}

pub struct Config {
    ips: i32,
    beep_frequency: f32,
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

    pub fn load_program(&mut self, path: String) -> () {
        // Taking in a vector of bytes from a file
        let program = std::fs::read(path).unwrap();
        // Copying the program data into memory starting from Byte 512 (0x200)
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program[..]);
        self.pc = 0x200; // Initializing program counter
    }

    // Initializing CHIP-8
    pub fn new() -> Arc<Mutex<CHIP8>> {
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

        Arc::new(Mutex::new(CHIP8 {
            memory: memory,
            pc: 0x00,
            index: 0x00,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variables: [0; 16],
            display: [[false; 32]; 64],
            renderer: ChipRender::setup().expect("Failed to initialize chip display renderer"),
            config: Config { ips: 700, beep_frequency: 34.648 },
        }))

    }

}

pub fn run(chip8:&Arc<Mutex<CHIP8>>) -> () {

    // Setting up keyboard interface mappings of key inputs to hex values 0x0-0xF
    let keys: [Keycode; 16] =  
    [ X, Key1, Key2, Key3, Q, W, E, A, S, D, Z, C, Key4, R, F, V ];
    let device_state = DeviceState::new();
    
    loop {
        let mut chip8 = chip8.lock().unwrap();
        let simulated_execution_time = 1.0/(chip8.config.ips as f32);
        // Calculating desired time to sleep between instructions using desired IPS
        // Sleeping the calculated time
        spin_sleep::sleep(Duration::from_secs_f32(simulated_execution_time));
        
        // Update the vector of Keycodes corresponding to keys currently being pressed
        let pressed_keys: Vec<Keycode> = device_state.get_keys();
        if pressed_keys.contains(&Keycode::Escape){
            panic!("Escape key pressed, program exited.");
        }
        // FETCH STAGE
        // Fetch the instruction from the program counter
        let inst_part1 = chip8.mem_read(chip8.pc);
        let inst_part2 = chip8.mem_read(chip8.pc + 1);
        // Combining both parts of the instruction to make the true instruction
        // using bit-wise or and padding with zeros.
        let instruction: u16 = ((inst_part1 as u16) << 8) | inst_part2 as u16;
        // Increment the program counter by 2
        chip8.pc = chip8.pc + 2;

        // DECODE STAGE
        // Extracting information from the instruction half-bytes (nibbles or nybbles, lol)
        // each nibble numbered (1-4) encodes information to be decoded before execution.
        let op = (instruction & 0xF000) >> 12;         // Opcode, encodes which type of instruction
        let x = ((instruction & 0x0F00) >> 8) as u8;    // Used to look up a variable register (0x0-0xf)
        let y = ((instruction & 0x00F0) >> 4) as u8;    // Also used to look up a variable register (0x0-0xf)
        let n = instruction & 0x000F;                  // 4-bit  number
        let byte = (instruction & 0x00FF) as u8;        // 8-bit  immediate number
        let addr = instruction & 0x0FFF;               // 12-bit immediate memory address
        // EXECUTE STAGE
        // This match statement contains all the instruction logic that can be executed by the CHIP-8,
        // implemented according to their original corresponding functionality.
        log::info!("op:{} x:{} y:{} n:{} byte:{} addr:{}",op,x,y,n,byte,addr);
        let display = chip8.display.clone();
        chip8.renderer.render(& mut display.clone());
        match op {
            
            0 => {
                match n{
                    // Clear screen (00E0)
                    0 => {
                        chip8.display = [[false; 32]; 64];
                        
                      
                    }
                    // 00EE subroutine
                    _=>{
                        chip8.pc = chip8.stack[chip8.stack.len()-1];
                        chip8.stack.pop();
                      
                        
                    }
                }
            }

            // Jump (1NNN)
            1 => {
                chip8.pc = addr;
                
            }
            // Set register vx (6XNN)
            //2NNN Subroutine
            2 =>{
                let stack_pc = chip8.pc.clone();
                chip8.stack.push(stack_pc);
                chip8.pc = addr;
               
            }

            6 => {
                chip8.variables[x as usize] = byte;
                
            }
            // Add value to register vx (7XNN)
            7 => {
                chip8.variables[x as usize] = chip8.variables[x as usize].wrapping_add(byte);
               
            }
            // Set index register I (ANNN)
            0xA => {
                chip8.index = addr;
               
            }
            // Draw to screen  (DXYN)
            0xD => {
                // Getting the co-ordinates to be drawn to from registers vx and vy
                let x_pos = ((chip8.variables[x as usize]) % 64) as usize; 
                let y_pos = ((chip8.variables[y as usize]) % 32) as usize; 
                //x_pos = ((x_pos % 64)+ 64) % 64 ; // x mod 64
                //y_pos = ((y_pos % 32)+ 32) % 32 ; // y mod 32
                let mut current_y_pos = y_pos;
               
                // Setting flag register to zero.
                chip8.variables[15] = 0x0;
                // For "n" rows on the screen
                for row in 0..n {
                    let mut current_x_pos = x_pos;
                    // Getting a row of sprite data from the address stored in Index (I)
                    // This is the n'th sprite data byte corresponding to the n'th row
                    let row_of_sprite_data = chip8.memory[chip8.index as usize + row as usize];
                    // Iterating over the bits in the chosen sprite byte as boolean value "bit"
                    for bit in (0..8).rev().map(|i| (row_of_sprite_data >> i) & 1 == 1) {
                        // If the bit on screen is on and the pixel of the sprite is on, turn
                        // off the pixel and set the flag register (vf) to 1.
                        if bit == true && chip8.display[current_x_pos][current_y_pos] == true {
                            chip8.display[current_x_pos][current_y_pos] = false; // Turning off the pixel
                            chip8.variables[15] = 1; // Setting vf to 1
                        }
                        // Otherwise if the bit on the screen is off and the pixel sprite is on,
                        // turn on the pixel and set the flag register (vf) to 1.
                        else if bit == true && chip8.display[current_x_pos][current_y_pos] == false{
                            // Turning on the pixel
                            chip8.display[current_x_pos][current_y_pos] = true;  
                        }
                        // If we reach the right edge of the screen, stop drawing the current row
                        if current_x_pos == 63 {
                            log::info!("Broke @ x = {}.",current_x_pos);
                        
                            break;
                        }
                        // Otherwise we continue and increment the x position of the current pixel on the display
                        current_x_pos = current_x_pos + 1;
                        
                    }
                    // Stop if we reached the bottom of the screen
                    if current_y_pos == 31 {
                        log::info!("Broke @ y = {}.",current_y_pos);
                        
                        break;
                    }
                    log::info!("Drew line @ y = {}.",current_y_pos);
                    current_y_pos = current_y_pos + 1;
                    
                }
                // Render the sprite on screen
                log::info!("Exiting from draw instruction succesfully.");
                
                log::info!("Exiting from rendering display succesfully.");
                
            }

            // Skip Instuctions
            // 3XNN - Skip one instruction if the value in Vx is equal to NN
            3 => {
                if chip8.variables[x as usize] == byte {
                    chip8.pc = chip8.pc + 2;
                   
                }
            }
            // 4XNN - Skip one instruction if the value in Vx is NOT equal to NN
            4 => {
                if chip8.variables[x as usize] != byte {
                    chip8.pc = chip8.pc + 2;
                    
                }
            }
            // 5XY0 - Skip one instruction if Vx and Vy are equal.
            5 => {
                if chip8.variables[x as usize] == chip8.variables[y as usize] {
                    chip8.pc = chip8.pc + 2;
                   
                }
            }
            // 9XY0 - Skip one instruction if Vx and Vy are NOT equal.
            9 => {
                if chip8.variables[x as usize] != chip8.variables[y as usize] {
                    chip8.pc = chip8.pc + 2;
                }
            }

            // Logical and arithmetic instructions
            8 => {
                match n {
                    // 8X70 Set - Vx is set to the value in Vy
                    0 => {
                        chip8.variables[x as usize] = chip8.variables[y as usize];
                        
                    }
                    // 8X71 Binary OR - Vx = Vx OR Vy
                    1 => {
                        chip8.variables[x as usize] =
                        chip8.variables[x as usize] | chip8.variables[y as usize];
                    }
                    // 8XY2 Binary AND - Vx = Vx AND Vy
                    2 => {
                        chip8.variables[x as usize] =
                        chip8.variables[x as usize] & chip8.variables[y as usize];
                    }
                    // 8XY3 Logical XOR - Vx = Vx XOR Vy
                    3 => {
                        chip8.variables[x as usize] =
                        chip8.variables[x as usize] ^ chip8.variables[y as usize];
                    }
                    // 8XY4 Add - Vx = Vx + Vy (with overflow flag)
                    4 => {
                        // If the addition would result in an overflow
                        if chip8.variables[x as usize] as u16 + chip8.variables[y as usize] as u16 > 255
                        {
                            // Set flag register to 1.
                            chip8.variables[15] = 1;
                        }
                        chip8.variables[x as usize] =
                        chip8.variables[x as usize].wrapping_add(chip8.variables[y as usize]);
                    }
                    // 8XY5 Subtract - Vx = Vx - Vy
                    5 => {
                        // Set flag to 1
                        chip8.variables[15] = 1;
                        // If the result will underflow, set flag to zero.
                        if chip8.variables[x as usize] < chip8.variables[y as usize] {
                            chip8.variables[15] = 0;
                        }
                        chip8.variables[x as usize] = chip8.variables[x as usize].wrapping_sub(chip8.variables[y as usize]);
                    }
                    // 8XY6 Shift Right (Ambiguous Instruction)
                    6 => {
                        // (Optional Step) Set Vx to Vy
                        //self.variables[x as usize] = self.variables[y as usize];
                        // Set flag bit to the bit that will get shifted out
                        chip8.variables[15] = chip8.variables[x as usize] & 0x01;
                        // Shift Vx one to the right
                        chip8.variables[x as usize] = chip8.variables[x as usize] >> 1;
                    }
                    // 8XY7 Subtract - Vx = Vy - Vx
                    7 => {
                        // Set flag to 1
                        chip8.variables[15] = 1;
                        // If the result will underflow, set flag to zero.
                        if chip8.variables[y as usize] < chip8.variables[x as usize] {
                            chip8.variables[15] = 0;
                        }
                        chip8.variables[x as usize] =
                        chip8.variables[y as usize].overflowing_sub(chip8.variables[x as usize]).0;
                    }
                    // 8XYE Shift Left (Ambiguous Instruction)
                    0xE => {
                        // (Optional Step) Set Vx to Vy
                        //self.variables[x as usize] = self.variables[y as usize];
                        // Set flag bit to the bit that will get shifted out
                        chip8.variables[15] = (chip8.variables[x as usize] >> 7) & 0x1;
                        // Shift Vx one to the left
                        chip8.variables[x as usize] = chip8.variables[x as usize] << 1;
                    }

                    _ => {
                        panic!("Opcode {} n = {} not found", op, n);
                    }
                }
            }
            //BNNN Jump With Offset (Ambigious Instruction)
            0xB => {
                // Jump to the address NNN + the value in v0
                chip8.pc = addr + chip8.variables[0] as u16;
            }
            // CXNN Random
            0xC => {
                // Generate a random number and AND it with NN and store result in Vx
                let random_u8: u8 = rand::thread_rng().gen();
                chip8.variables[x as usize] = byte & random_u8;
            }
            // Skip if Key Instructions
            0xE => {
                match byte {
                    // Skip an instruction if key in Vx is pressed
                    0x9E => {
                        log::info!("Skipping if key: {} is pressed.",chip8.variables[x as usize]);
                       
                        if pressed_keys.contains(&keys[chip8.variables[x as usize] as usize]){
                            chip8.pc = chip8.pc + 2;
                        }
                    }
                    // Skip an instruction if key in Vx is NOT pressed
                    0xA1 => {
                        log::info!("Skipping if key: {} is NOT pressed.",chip8.variables[x as usize]);
                        
                        if !pressed_keys.contains(&keys[chip8.variables[x as usize] as usize]){
                            chip8.pc = chip8.pc + 2;
                        }
                    }

                    _ => {
                        panic!("Opcode {} n = {} not found", op, n);
                    }
                }
            }
            // Misc Instructions
            0xF => {
                match byte {
                    // FX07 Sets Vx to current value of delay timer
                    0x07 => {
                        chip8.variables[x as usize] = chip8.delay_timer;
                        log::info!("Delay_timer = {}",chip8.delay_timer);
                        log::info!("Vx = {}",chip8.variables[x as usize]);
                    }

                    // FX15 Sets the delay timer to the value in Vx
                    0x15 => {
                        chip8.delay_timer = chip8.variables[ x as usize];
                    }

                    // FX18 Sets the sound timer to the value in Vx
                    0x18 => {
                        chip8.sound_timer = chip8.variables[x as usize];
                    }

                    // FX1E Add to index
                    0x1E => {
                        chip8.index = chip8.index + chip8.variables[x as usize] as u16;
                    }

                    // FX0A Get key
                    0x0A => {
                        log::info!("Waiting on key press...");
                        
                            // If a key is pressed, store that keycode and exit the loop
                                if pressed_keys.len() != 0{
                                    chip8.variables[x as usize] = match pressed_keys[0]{
                                        Key1 => {0x1}
                                        Key2 => {0x2}
                                        Key3 => {0x3}
                                        Key4 => {0xC}
                                        Q => {0x4}
                                        W => {0x5}
                                        E => {0x6}
                                        R => {0xD}
                                        A => {0x7}
                                        S => {0x8}
                                        D => {0x9}
                                        F => {0xE}
                                        Z => {0xA}
                                        X => {0x0}
                                        C => {0xB}
                                        V => {0xF}
                                        _=> panic!("Could not find key {}",pressed_keys[0])
                                    };
                            log::info!("Captured key input {}",chip8.variables[x as usize]);
                            
                            }
                            else{
                                chip8.pc = chip8.pc - 2;
                            }
                    }

                    // FX29 Font character
                    0x29 => {
                        // Setting the index register to the address of the font character in Vx
                        chip8.index = 0x050 as u16 + ( chip8.variables[x as usize] * 5 ) as u16
                    }

                    // FX33 Binary-coded decimal conversion
                    0x33 => {
                        // This should take the number in Vx (0-255) and convert it to 3 digits 
                        // and stores them in memory starting at the address in the index register.
                        let mut number = chip8.variables[x as usize];
                        for i in 0..3{
                            let digit = (number) % 10;
                            number = number / 10;
                            let chip8_index = chip8.index.clone();
                            chip8.memory[(chip8_index as usize + 2 - i )] = digit;
                            
                        }
                    }

                    // FX55 Store memory (Ambiguous Instruction)
                    0x55 => {
                        //println!("STORED");
                        for i in 0..(x as usize)+1{
                            let chip8_index = chip8.index.clone();
                            chip8.memory[chip8_index as usize + i] = chip8.variables[i];
                        }
                    }

                    // FX65 Load memory (Ambiguous Instruction)
                    0x65 => {
                        //println!("LOADED");
                        for i in 0..(x as usize)+1{
                            chip8.variables[i] = chip8.memory[chip8.index as usize + i];
                        }

                    }

                    _ => {
                        panic!("Opcode {} n = {} not found", op, n);
                    }   
                }
            }

            // Catch-all for unrecognized instructions
            _ => {
                panic!("Opcode {} not found", op);
            }
        }
        
        drop(chip8);
    }
}