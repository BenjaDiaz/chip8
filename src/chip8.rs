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
    opcode: u16,
    memory: [u8; 4096],
    /// CPU registers
    v: [u8; 16],
    /// Index register
    i: u16,
    /// Program counter
    pc: usize,
    /// 2048 pixels graphics
    gfx: [u8; 64 * 32],
    /// 60hz timer
    delay_timer: u8,
    /// 60hz timer
    sound_timer: u8,
    /// Stores program counter
    stack: [u16; 16],
    /// Stack pointer
    sp: usize,
    /// HEX based keypad (0x0 - 0xF)
    keypad: [u8; 16],
    /// Defines the need to update the screen. Set by opcode 0x00E0 (clear) or 0xDXYN (draw)
    draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        println!("new()");
        let mut memory = [0; 4096];
        for x in 0..80 {
            memory[x] = CHIP8_FONTSET[x];
        }
        Chip8 {
            opcode: 0,
            memory,
            v: [0; 16],
            i: 0,
            pc: 0x200, // 80
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
            draw_flag: false,
        }
    }

    pub fn cycle(&mut self) {
        println!("cycle()");
        self.opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;

        match self.opcode & 0xF000 {
            /// 0xANNN
            /// Sets I to the address NNN.
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            /// 0x2NNN
            /// Calls subroutine at NNN.
            0x2000 => {
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = (self.opcode & 0x0FFF) as usize;
            }
            /// 0xDXYN
            /// Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            /// Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction.
            /// As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen.
            0xD000 => {
                let x = V[self.opcode & 0x0F00];
                let y = V[self.opcode & 0x00F0];
                let h = self.opcode & 0x000F;
                let pixel;

                V[0xF] = 0;
                for yline in 0..h {
                    pixel = self.memory[self.i + yline];
                    for xline in 0..8 {
                        if pixel & (0b10000000 >> xline) != 0 {
                            // Check if there is already a pixel drawn
                            if self.gfx[x + xline + ((y + yline) * 64)] {
                                V[0xF] = 1
                            }
                            // Set the pixel value by using XOR
                            self.gfx[x + xline + ((y + yline) * 64)] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
                self.pc += 2;
            }
            _ => {}
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

    pub fn should_draw(&mut self) -> bool {
        self.draw_flag
    }

    pub fn draw(&mut self) {}
}