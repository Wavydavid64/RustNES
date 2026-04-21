use std::io::{Error, ErrorKind};

const PROGRAM_ROM_UNIT: usize = 16 * 1024;
const CHARACTER_ROM_UNIT: usize = 8 * 1024;

const PROGRAM_RAM_UNIT: usize = 8 * 1024;

const HEADER_SIZE_BYTES: usize = 16;
const NES_HEADER: [u8; 4] = [0x4e, 0x45, 0x53, 0x1A];

#[derive(Default, PartialEq)]
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
    VsUnisystem,
}

#[derive(Default)]
struct Metadata {
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
    let metadata = process_header(header)?;
    Ok(())
}

fn process_header(header: &[u8]) -> Result<Metadata, Error> {
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

    let format = get_format(header);

    let mut metadata = Metadata {
        format,
        program_rom_bytes: PROGRAM_ROM_UNIT * (header[4] as usize),
        character_rom_bytes: CHARACTER_ROM_UNIT * (header[5] as usize),
        ..Default::default()
    };

    let mut mapper = process_byte_6(&mut metadata, header[6]);
    mapper |= process_byte_7(&mut metadata, header[7]);
    metadata.mapper = mapper;

    process_byte_8(&mut metadata, header[8]);
    process_byte_9(&mut metadata, header[9]);

    Ok(metadata)
}

fn get_format(header: &[u8]) -> Format {
    let decision_flag = header[7] & 0xc;
    match decision_flag {
        0x0 => {
            if header[12..=15].iter().all(|byte| *byte == 0) {
                Format::INes
            } else {
                Format::ArchaicINes
            }
        }
        _ => Format::ArchaicINes,
    }
}

fn process_byte_6(metadata: &mut Metadata, byte: u8) -> u8 {
    let nametable_arrangement_bit = byte & 1 != 0;
    metadata.mirroring_type = if nametable_arrangement_bit {
        MirroringType::Horizontal
    } else {
        MirroringType::Vertical
    };
    metadata.has_battery = (byte & (1 << 1)) != 0;
    metadata.has_trainer = (byte & (1 << 2)) != 0;
    metadata.has_alternative_nametable_layout = (byte & (1 << 3)) != 0;
    (byte & (0b11110000)) >> 4
}

fn process_byte_7(metadata: &mut Metadata, byte: u8) -> u8 {
    if metadata.format == Format::ArchaicINes {
        return 0;
    }
    metadata.console_type = if byte & 1 != 0 {
        ConsoleType::VsUnisystem
    } else {
        ConsoleType::Nes
    };
    byte & 0b11110000
}

fn process_byte_8(metadata: &mut Metadata, byte: u8) {
    metadata.program_ram_bytes = PROGRAM_RAM_UNIT * (byte as usize)
}

fn process_byte_9(metadata: &mut Metadata, byte: u8) {
    if metadata.format == Format::ArchaicINes {
        return;
    }
    metadata.tv_system = if byte & 1 != 0 {
        TvSystem::Pal
    } else {
        TvSystem::Ntsc
    };
}
