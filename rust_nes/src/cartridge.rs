use std::io::{Error, ErrorKind};

const PROGRAM_ROM_SIZE_UNIT: usize = 16 * 1024;
const CHARCTER_ROM_SIZE_UNIT: usize = 8 * 1024;

const HEADER_SIZE_BYTES: usize = 16;
const NES_HEADER: [u8; 4] = [0x4e, 0x45, 0x53, 0x1A];

#[derive(Default)]
enum Format {
    #[default]
    ArchaicINes,
    INes,
}

#[derive(Default)]
enum TvSystem {
    #[default]
    Ntsc,
    Pal,
}

#[derive(Default)]
enum MirroringType {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default)]
enum ConsoleType {
    #[default]
    Nes,
    NesVs,
    Playchoice10,
}

#[derive(Default)]
struct Header {
    format: Format,
    program_rom_bytes: usize,
    character_rom_bytes: usize,
    program_ram_bytes: usize,
    character_ram_bytes: usize,
    mapper: u8,
    mirroring_type: MirroringType,
    has_trainer: bool,
    has_battery: bool,
    has_alternative_nametable_layout: bool,
    console_type: ConsoleType,
    tv_system: TvSystem,
}

pub fn load_nes_file(filepath: &str) -> Result<(), Error> {
    let bytes = std::fs::read(filepath)?;
    let header = &bytes[..HEADER_SIZE_BYTES];
    let header = process_header(header);
    Ok(())
}

fn process_header(header: &[u8]) -> Result<Header, Error> {
    if header.len() != HEADER_SIZE_BYTES {
        let error_msg = format!(
            "Header is of wrong size! (Expected: {} Actual: {})",
            HEADER_SIZE_BYTES,
            header.len(),
        );
        return Err(Error::new(ErrorKind::InvalidData, error_msg));
    }

    if !header.starts_with(&NES_HEADER) {
        let error_msg = format!(
            "Header is missing expected constant at beginning of header! (Expected: {:02X?} Actual: {:02X?})",
            NES_HEADER,
            &header[..NES_HEADER.len()]
        );
        return Err(Error::new(ErrorKind::InvalidData, error_msg));
    }

    Ok(Header {
        ..Default::default()
    })
}
