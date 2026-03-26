use crate::system::bus::BusError;

pub struct PpuMemory {
    // --- Memory ---
    pub(super) vram: [u8; 8192],
    pub(super) oam: [u8; 160],

    // --- Registers ---
    pub(super) lcdc: u8,      // 0xFF40
    pub(super) stat: u8,      // 0xFF41
    pub(super) scy: u8,       // 0xFF42
    pub(super) scx: u8,       // 0xFF43
    pub(super) ly: u8,        // 0xFF44
    pub(super) lyc: u8,       // 0xFF45
    pub(super) bgp: [u8; 4],  // 0xFF47
    pub(super) obp0: [u8; 4], // 0xFF48
    pub(super) obp1: [u8; 4], // 0xFF49
    pub(super) wy: u8,        // 0xFF4A
    pub(super) wx: u8,        // 0xFF4B
}

impl PpuMemory {
    pub fn new() -> Self {
        Self {
            vram: [0; 8192],
            oam: [0; 160],

            // --- Registers ---
            lcdc: 0x91,
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: [0, 3, 3, 3],
            obp0: [0, 3, 3, 3],
            obp1: [0, 3, 3, 3],
            wy: 0,
            wx: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0x8000..=0x9FFF => Ok(self.vram[(addr - 0x8000) as usize]),
            0xFE00..=0xFE9F => Ok(self.oam[(addr - 0xFE00) as usize]),
            0xFF40 => Ok(self.lcdc),
            0xFF41 => Ok(self.stat),
            0xFF42 => Ok(self.scy),
            0xFF43 => Ok(self.scx),
            0xFF44 => Ok(self.ly),
            0xFF45 => Ok(self.lyc),
            0xFF4A => Ok(self.wy),
            0xFF4B => Ok(self.wx),
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = val,
            0xFF40 => {
                self.lcdc = val;
                if val & 0x80 == 0 {
                    self.ly = 0;
                    self.stat = self.stat & 0b11111100;
                }
            }
            0xFF41 => self.stat = (val & 0b11111000) | 0b10000000,
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = 0,
            0xFF45 => self.lyc = val,
            0xFF47 => {
                self.bgp = [
                    val & 0b11,
                    (val & 0b1100) >> 2,
                    (val & 0b110000) >> 4,
                    (val & 0b11000000) >> 6,
                ]
            }
            0xFF48 => {
                self.obp0 = [
                    val & 0b11,
                    (val & 0b1100) >> 2,
                    (val & 0b110000) >> 4,
                    (val & 0b11000000) >> 6,
                ]
            }
            0xFF49 => {
                self.obp1 = [
                    val & 0b11,
                    (val & 0b1100) >> 2,
                    (val & 0b110000) >> 4,
                    (val & 0b11000000) >> 6,
                ]
            }
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ => return Ok(()),
        }
        Ok(())
    }
}
