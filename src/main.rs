use chip8::Chip8;

mod chip8;


fn main() {
    let mut chip8 = Chip8::new();
    loop {
        if chip8.should_draw() {
            chip8.draw()
        }
        chip8.cycle();

        chip8.set_keys();
    }
}
