mod cartridge;

use crate::cartridge::load_nes_file;

fn main() {
    let filepath = "/Users/david/Desktop/code/chip8/src/programs/15puzzle.rom";
    load_nes_file(filepath);
}
