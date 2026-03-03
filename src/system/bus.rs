mod io;
mod rom;

use rom::{Rom, RomError};
use std::{error::Error, fmt};

use io::Io;

pub enum BusError {
    RomError(RomError),
    OutOfRange(u16),
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
            BusError::OutOfRange(addr) => write!(f, "bus -> Address {:#04X} out of bounds", addr),
        }
    }
}

pub struct Bus {
    rom: Rom,
    vram: [u8; 8192],
    wram: [u8; 8192],
    oam: [u8; 160],
    io: Io,
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
            io: Io::new(),
            hram: [0; 127],
            ie_reg: 0,
        }
    }

    // Load a rom file into memory
    pub fn load_rom(&mut self, rom_fname: &str) -> Result<(), Box<dyn Error>> {
        self.rom.load_rom(rom_fname)
    }

    // Read a single byte from memory
    // 0x0000-7FFF - ROM
    // 0x8000-9FFF - VRAM
    // 0xA000-BFFF - RAM
    // 0xC000-DFFF - WRAM
    // 0xE000-FDFF - Mirror of 0xC000-DDFF
    // 0xFE00-FE9F - Object attribute memory
    // 0xFEA0-FEFF - Not Usable
    // 0xFF00-FF7F - I/O Registers
    // 0xFF80-FFFE - High RAM
    // 0xFFFF      - IE register
    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.rom.read_byte(addr).map_err(BusError::from),
            0x8000..=0x9FFF => Ok(self.vram[(addr - 0x8000) as usize]),
            0xC000..=0xDFFF => Ok(self.wram[(addr - 0xC000) as usize]),
            0xE000..=0xFDFF => Ok(self.wram[(addr - 0xC000 - 0x2000) as usize]),
            0xFE00..=0xFE9F => Ok(self.oam[(addr - 0xFE00) as usize]),
            0xFEA0..=0xFEFF => Ok(0xFF),
            0xFF00..=0xFF7F => self.io.read_reg(addr),
            0xFF80..=0xFFFE => Ok(self.hram[(addr - 0xFF80) as usize]),
            0xFFFF => Ok(self.ie_reg),
        }
    }

    // Write a single byte to memory
    // Returns Err if trying to write to a read-only range
    // 0x0000-7FFF - ROM
    // 0x8000-9FFF - VRAM
    // 0xA000-BFFF - RAM
    // 0xC000-DFFF - WRAM
    // 0xE000-FDFF - Mirror of 0xC000-DDFF
    // 0xFE00-FE9F - Object attribute memory
    // 0xFEA0-FEFF - Not Usable
    // 0xFF00-FF7F - I/O Registers
    // 0xFF80-FFFE - High RAM
    // 0xFFFF      - IE register
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
            0xE000..=0xFDFF => {
                self.wram[(addr - 0xC000 - 0x2000) as usize] = val;
                Ok(())
            }
            0xFE00..=0xFE9F => {
                self.oam[(addr - 0xFE00) as usize] = val;
                Ok(())
            }
            0xFEA0..=0xFEFF => Ok(()),
            0xFF00..=0xFF7F => self.io.write_reg(addr, val),
            0xFF80..=0xFFFE => {
                self.hram[(addr - 0xFF80) as usize] = val;
                Ok(())
            }
            0xFFFF => {
                self.ie_reg = val;
                Ok(())
            }
        }
    }

    pub fn tick_timers(&mut self) {
        self.io.tick_timers();
    }

    pub fn check_timer_interrupt(&mut self) -> bool {
        self.io.check_timer_interrupt()
    }
}
