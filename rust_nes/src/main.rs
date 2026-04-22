mod cartridge;

use crate::cartridge::rom::Cartridge;

fn main() {
    let filepath = "/Users/david/Desktop/code/rustnes/rust_nes/src/roms/nestest.nes";
    let cartridge = Cartridge::new(filepath);
}
