pub struct ChrMemory {
    ram: Vec<u8>,
    rom: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
pub enum ChrTarget {
    Ram(usize),
    Rom(usize),
    OpenBus,
}

impl ChrMemory {
    pub fn new(ram: Vec<u8>, rom: Vec<u8>) -> Self {
        Self { ram, rom }
    }

    pub fn read(&self, target: ChrTarget) -> u8 {
        match target {
            ChrTarget::Ram(offset) => self.ram[offset],
            ChrTarget::Rom(offset) => self.rom[offset],
            ChrTarget::OpenBus => 0,
        }
    }

    pub fn write(&mut self, target: ChrTarget, value: u8) {
        match target {
            ChrTarget::Ram(offset) => self.ram[offset] = value,
            _ => {}
        }
    }
}
