pub enum Mapper {
    Nrom,
}

impl Mapper {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Nrom),
            _ => None,
        }
    }
}
