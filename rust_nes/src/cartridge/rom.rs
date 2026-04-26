use crate::cartridge::chr_memory::ChrMemory;
use crate::cartridge::header::{process_header, Metadata, HEADER_SIZE};
use crate::cartridge::mapper::Mapper;
use std::io::Error;

const TRAINER_SIZE: usize = 512; // bytes

pub struct Cartridge {
    metadata: Metadata,
    trainer: Option<Vec<u8>>,
    prg_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    chr_memory: ChrMemory,
    mapper: Mapper,
}

impl Cartridge {
    pub fn new(filepath: &str) -> Self {
        load_rom(filepath).expect("Failed to load rom!")
    }
}

fn load_rom(filepath: &str) -> Result<Cartridge, Error> {
    let bytes = std::fs::read(filepath)?;

    let mut bytes_offset = 0;

    let header = &bytes[..HEADER_SIZE];
    let metadata = process_header(header)?;
    bytes_offset += HEADER_SIZE;

    let trainer = if metadata.has_trainer {
        let trainer = bytes[bytes_offset..bytes_offset + TRAINER_SIZE].to_vec();
        bytes_offset += TRAINER_SIZE;
        Some(trainer)
    } else {
        None
    };

    let prg_rom = bytes[bytes_offset..bytes_offset + metadata.prg_rom_bytes].to_vec();
    bytes_offset += metadata.prg_rom_bytes;

    let prg_ram = vec![0; metadata.prg_ram_bytes];

    let (chr_rom, chr_ram) = if metadata.chr_rom_bytes > 0 {
        let chr_rom: Vec<u8> = bytes[bytes_offset..bytes_offset + metadata.chr_rom_bytes].to_vec();
        // No chr ram if rom exists
        (chr_rom, Vec::new() as Vec<u8>)
    } else {
        let chr_ram: Vec<u8> = vec![0; metadata.chr_ram_bytes];
        // No chr rom if ram exists
        (Vec::new() as Vec<u8>, chr_ram)
    };

    let chr_memory = ChrMemory::new(chr_ram, chr_rom);

    let mapper = Mapper::from_id(metadata.mapper).expect("Unsupported mapper!");

    Ok(Cartridge {
        metadata,
        trainer,
        prg_rom,
        prg_ram,
        chr_memory,
        mapper,
    })
}
