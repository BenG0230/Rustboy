mod mbc0;
mod mbc1;

use mbc0::Mbc0;
use mbc1::Mbc1;

pub enum Mbc {
    Mbc0(Mbc0),
    Mbc1(Mbc1),
}

pub trait MbcTrait {
    fn read_byte(&self, addr: u16) -> Result<u8, super::RomError>;
    fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), super::RomError>;
}

impl Mbc {
    pub fn new(rom_data: Vec<u8>) -> Self {
        match rom_data[0x147] {
            0x00 => Self::Mbc0(Mbc0::new(rom_data)),
            0x01 | 0x02 | 0x03 => Self::Mbc1(Mbc1::new(rom_data)),
            _ => panic!("Unknown mbc"),
        }
    }
}

impl MbcTrait for Mbc {
    fn read_byte(&self, addr: u16) -> Result<u8, super::RomError> {
        match self {
            Mbc::Mbc0(mbc) => mbc.read_byte(addr),
            Mbc::Mbc1(mbc) => mbc.read_byte(addr),
            _ => Err(super::RomError::NoROM),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), super::RomError> {
        match self {
            Mbc::Mbc0(mbc) => mbc.write_byte(addr, val),
            Mbc::Mbc1(mbc) => mbc.write_byte(addr, val),
            _ => Err(super::RomError::NoROM),
        }
    }
}
