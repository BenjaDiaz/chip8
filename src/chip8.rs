use crate::drivers::DisplayDriver;
use std::path::PathBuf;

const CHIP8_FONTSET: [u8; 80] =
    [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

pub struct Chip8 {
    /// 8bit memory
    memory: [u8; 4096],
    /// CPU registers
    v: [u8; 16],
    /// Index register
    i: usize,
    /// Program counter
    pc: usize,
    /// 2048 pixels graphics
    gfx: [u8; 64 * 32],
    /// 60hz timer
    delay_timer: u8,
    /// 60hz timer
    sound_timer: u8,
    /// Stores program counter
    stack: [usize; 16],
    /// Stack pointer
    sp: usize,
    /// HEX based keypad (0x0 - 0xF)
    keypad: [u8; 16],
    /// Defines the need to update the screen. Set by opcode 0x00E0 (clear) or 0xDXYN (draw)
    pub draw_flag: bool,
    /// Responsible for handling window and drawing pixels defined in gfx
    display: DisplayDriver,
}

impl Chip8 {
    pub fn new(display: DisplayDriver) -> Chip8 {
        println!("new()");
        let mut memory = [0; 4096];
        for x in 0..80 {
            memory[x] = CHIP8_FONTSET[x];
        }
        Chip8 {
            memory,
            v: [0; 16],
            i: 0,
            pc: 0x200, // 512
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
            draw_flag: true,
            display,
        }
    }

    pub fn cycle(&mut self) {
        println!("cycle()");
        let opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;

        match opcode & 0xF000 {
            // 0x0000
            0x0000 => {
                match opcode {
                    // 0x00E0
                    // Clears the screen
                    0x00E0 => {
                        self.display.clear();
                        self.pc += 2;
                    }
                    // 0x00EE
                    // Returns from a subroutine.
                    0x00EE => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                    }
                    _ => { println!("Unknown opcode 0x{:02x}", opcode) }
                }
            }
            // 0x1NNN
            // Jumps to address NNN.
            0x1000 => {
                self.pc = (opcode & 0x0FFF) as usize;
            }

            // 0x2NNN
            // Calls subroutine at NNN.
            0x2000 => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = (opcode & 0x0FFF) as usize;
            }

            // 0x3XNN
            // Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block).
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = opcode & 0x00FF;
                if self.v[x] == nn as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            // 0x4XNN
            // Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = opcode & 0x00FF;
                if self.v[x] != nn as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // 0x5XY0
            // Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            // 0x6XNN
            // Sets VX to NN.
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = nn;
                self.pc += 2;
            }
            // 0x7XNN
            // Adds NN to VX. (Carry flag is not changed)
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] += nn;
                self.pc += 2;
            }

            // 0xANNN
            // Sets I to the address NNN.
            0xA000 => {
                self.i = (opcode & 0x0FFF) as usize;
                self.pc += 2;
            }

            // 0xDXYN
            // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            // Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction.
            // As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen.
            0xD000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let vx = self.v[((opcode & 0x0F00) >> 8) as usize];
                let vy = self.v[((opcode & 0x00F0) >> 4) as usize];
                let h = (opcode & 0x000F) as u8;
                let mut pixel;
                let mut gfx_pos;

                self.v[0xF] = 0;
                for yline in 0..h {
                    pixel = self.memory[self.i + yline as usize];
                    for xline in 0..8 {
                        if pixel & (0b10000000 >> xline) != 0 {
                            // Check if there is already a pixel drawn
                            gfx_pos = ((vx + xline) as u32 + ((vy + yline) as u32 * 64)) as usize;
                            if self.gfx[gfx_pos] == 1 {
                                self.v[0xF] = 1
                            }
                            // Set the pixel value by using XOR
                            self.gfx[gfx_pos] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
                self.pc += 2;
            }
            _ => { println!("Unknown opcode 0x{:02x}", opcode) }
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!")
            }
            self.sound_timer -= 1;
        }
    }

    pub fn load(&mut self, path: PathBuf) {
        let rom = std::fs::read(path).unwrap();
        for (i, elem) in rom.iter().enumerate() {
            self.memory[0x200 + i] = *elem;
        }
    }

    pub fn draw(&mut self) {
        self.display.draw(self.gfx);
        self.draw_flag = false;
    }

    pub fn set_keys(&mut self) {}
}