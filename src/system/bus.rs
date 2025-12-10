mod rom;

use rom::{Rom, RomError};
use std::{error::Error, fmt};

pub enum BusError {
    RomError(RomError),
    OutOfBounds(u16),
}

impl From<RomError> for BusError {
    fn from(err: RomError) -> BusError {
        BusError::RomError(err)
    }
}

impl fmt::Display for BusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusError::RomError(err) => write!(f, "{}", err),
            BusError::OutOfBounds(addr) => write!(f, "Address {:#04X} out of bounds", addr),
        }
    }
}

pub struct Bus {
    rom: Rom,
    vram: [u8; 8192],
    wram: [u8; 8192],
    oam: [u8; 160],
    io_regs: [u8; 128],
    hram: [u8; 127],
    ie_reg: u8,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Rom::new(),
            vram: [0; 8192], //TODO: Update read/write for vram based on map
            wram: [0; 8192],
            oam: [0; 160],
            io_regs: [0; 128],
            hram: [0; 127],
            ie_reg: 0,
        }
    }

    pub fn load_rom(&mut self, rom_fname: &str) -> Result<(), Box<dyn Error>> {
        self.rom.load_rom(rom_fname)
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.rom.read_byte(addr).map_err(BusError::from),
            0x8000..=0x9FFF => Ok(self.vram[(addr - 0x8000) as usize]),
            0xC000..=0xDFFF => Ok(self.wram[(addr - 0xC000) as usize]),
            0xFE00..=0xFE9F => Ok(self.oam[(addr - 0xFE00) as usize]),
            0xFF00..=0xFF7F => Ok(self.io_regs[(addr - 0xFF00) as usize]),
            0xFF80..=0xFFFE => Ok(self.hram[(addr - 0xFF80) as usize]),
            0xFFFF => Ok(self.ie_reg),
            _ => Err(BusError::OutOfBounds(addr)),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => {
                self.rom.write_byte(addr, val).map_err(BusError::from)
            }
            0x8000..=0x9FFF => {
                self.vram[(addr - 0x8000) as usize] = val;
                Ok(())
            }
            0xC000..=0xDFFF => {
                self.wram[(addr - 0xC000) as usize] = val;
                Ok(())
            }
            0xFE00..=0xFE9F => {
                self.oam[(addr - 0xFE00) as usize] = val;
                Ok(())
            }
            0xFF00..=0xFF7F => {
                self.io_regs[(addr - 0xFF00) as usize] = val;
                Ok(())
            }
            0xFF80..=0xFFFE => {
                self.hram[(addr - 0xFF80) as usize] = val;
                Ok(())
            }
            0xFFFF => {
                self.ie_reg = val;
                Ok(())
            }
            _ => Err(BusError::OutOfBounds(addr)),
        }
    }
}
