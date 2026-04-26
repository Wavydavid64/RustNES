use std::io::{Error, ErrorKind};

const PRG_ROM_UNIT: usize = 16 * 1024;
const CHR_ROM_UNIT: usize = 8 * 1024;

const PRG_RAM_UNIT: usize = 8 * 1024;

const CHR_RAM_IF_NO_ROM: usize = 8 * 1024;

pub const HEADER_SIZE: usize = 16;
const NES_HEADER: [u8; 4] = [0x4e, 0x45, 0x53, 0x1A];

#[derive(Default, PartialEq, Debug)]
pub enum Format {
    #[default]
    ArchaicINes,
    INes,
}

#[derive(Default, Debug)]
pub enum TvSystem {
    #[default]
    Ntsc,
    Pal,
}

#[derive(Default, Debug)]
pub enum MirroringType {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default, Debug)]
pub enum ConsoleType {
    #[default]
    Nes,
    VsUnisystem,
}

#[derive(Default, Debug)]
pub struct Metadata {
    pub format: Format,
    pub prg_rom_bytes: usize,
    pub chr_rom_bytes: usize,
    pub prg_ram_bytes: usize,
    pub chr_ram_bytes: usize,
    pub mapper: u8,
    pub mirroring_type: MirroringType,
    pub has_trainer: bool,
    pub has_battery: bool,
    pub has_alternative_nametable_layout: bool,
    pub console_type: ConsoleType,
    pub tv_system: TvSystem,
}

pub fn process_header(header: &[u8]) -> Result<Metadata, Error> {
    if header.len() != HEADER_SIZE {
        let error_msg = format!(
            "Header is of wrong size! (Expected: {} Actual: {})",
            HEADER_SIZE,
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

    let chr_rom_bytes = CHR_ROM_UNIT * (header[5] as usize);
    let chr_ram_bytes = if chr_rom_bytes == 0 {
        CHR_RAM_IF_NO_ROM
    } else {
        0
    };

    let mut metadata = Metadata {
        format,
        prg_rom_bytes: PRG_ROM_UNIT * (header[4] as usize),
        chr_rom_bytes: chr_rom_bytes,
        chr_ram_bytes: chr_ram_bytes,
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
        MirroringType::Vertical
    } else {
        MirroringType::Horizontal
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
    metadata.prg_ram_bytes = PRG_RAM_UNIT * std::cmp::max(byte as usize, 1)
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
