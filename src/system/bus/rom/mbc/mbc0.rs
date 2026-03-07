use crate::system::bus::rom::RomError;

pub struct Mbc0 {
    rom: Vec<u8>,
}

impl Mbc0 {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self { rom: rom_data }
    }
}

impl super::MbcTrait for Mbc0 {
    // Read a single byte from ROM
    fn read_byte(&self, addr: u16) -> Result<u8, RomError> {
        match addr {
            0x0000..=0x7FFF => self
                .rom
                .get(addr as usize)
                .copied()
                .ok_or(RomError::OutOfBounds(addr)),
            _ => Err(RomError::OutOfBounds(addr)),
        }
    }

    // Does nothing :P
    fn write_byte(&mut self, _addr: u16, _val: u8) -> Result<(), RomError> {
        Ok(())
    }
}
