use crate::cartridge::header::{process_header, Metadata, HEADER_SIZE_BYTES};
use std::io::Error;

pub struct Cartridge {
    metadata: Metadata,
}

impl Cartridge {
    pub fn new(filepath: &str) -> Self {
        let metadata = load_rom(filepath).expect("Failed to load rom!");
        Self { metadata }
    }
}

pub fn load_rom(filepath: &str) -> Result<Metadata, Error> {
    let bytes = std::fs::read(filepath)?;
    let header = &bytes[..HEADER_SIZE_BYTES];
    let metadata = process_header(header)?;
    Ok(metadata)
}
