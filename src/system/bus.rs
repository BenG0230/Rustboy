mod io;
mod ppu;
mod rom;

use ppu::Ppu;
use rom::{Rom, RomError};
use std::{error::Error, fmt};

use io::Io;

#[derive(Debug)]
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
    ppu: Ppu,
    wram: [u8; 8192],
    io: Io,
    hram: [u8; 127],
    ie_reg: u8,
}

impl Bus {
    pub fn new(rom_fname: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            rom: Rom::new(rom_fname)?, // ROM + RAM
            ppu: Ppu::new(),           // VRAM + OAM + Video I/O Regs
            wram: [0; 8192],
            io: Io::new(),
            hram: [0; 127],
            ie_reg: 0,
        })
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
            0x8000..=0x9FFF | 0xFE00..=0xFE9F | 0xFF40..=0xFF4B => self.ppu.read_byte(addr),
            0xC000..=0xDFFF => Ok(self.wram[(addr - 0xC000) as usize]),
            0xE000..=0xFDFF => Ok(self.wram[(addr - 0xC000 - 0x2000) as usize]),
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
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.rom.write_byte(addr, val)?,
            0x8000..=0x9FFF | 0xFE00..=0xFE9F | 0xFF40..=0xFF4B => {
                if addr == 0xFF46 {
                    // DMA Transfer
                    for i in 0..160 {
                        let source = (val as u16) * 256 + i;
                        self.write_byte(0xFE00 + i, self.read_byte(source)?)?;
                    }
                }
                self.ppu.write_byte(addr, val)?
            }
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xE000..=0xFDFF => self.wram[(addr - 0xC000 - 0x2000) as usize] = val,
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => self.io.write_reg(addr, val)?,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            0xFFFF => self.ie_reg = val,
        }

        Ok(())
    }

    pub fn tick_timers(&mut self) {
        self.io.tick_timers();
    }

    pub fn step_ppu(&mut self) {
        self.ppu.step();
    }

    pub fn get_frame_buffer(&mut self) -> &mut Vec<u8> {
        self.ppu.get_frame_buffer()
    }

    pub fn check_timer_interrupt(&mut self) -> bool {
        self.io.check_timer_interrupt()
    }

    pub fn check_stat_interrupt(&mut self) -> bool {
        self.ppu.check_for_statinterrupt()
    }

    pub fn change_key(&mut self, button_index: usize, val: bool) {
        self.io.change_key(button_index, val);
    }

    pub fn check_vblank_interrupt(&mut self) -> bool {
        self.ppu.check_for_vblankinterrupt()
    }
}
