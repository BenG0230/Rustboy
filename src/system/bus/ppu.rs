use super::{Bus, BusError};

pub struct Ppu {
    palette: [u32; 4],
    timer: u32,

    // VRAM
    vram: [u8; 8192],

    // registers
    lcdc: u8, // 0xFF40
    stat: u8, // 0xFF41
    scy: u8,  // 0xFF42
    scx: u8,  // 0xFF43
    ly: u8,   // 0xFF44
    lyc: u8,  // 0xFF45
    wy: u8,   // 0xFF4A
    wx: u8,   // 0xFF4B

    // Emulator stuffs
    current_mode: u8,

    // flags
    requesting_interrupt: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            // #9a9e3f
            // #496b22
            // #0e450b
            // #1b2a09
            palette: [0x9a9e3f, 0x496b22, 0x0e450b, 0x1b2a09],
            timer: 0,
            vram: [0; 8192],
            lcdc: 0x91,
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            current_mode: 2,
            requesting_interrupt: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0x8000..=0x9FFF => Ok(self.vram[(addr - 0x8000) as usize]),
            0xFF40 => Ok(self.lcdc),
            0xFF41 => Ok(self.stat),
            0xFF42 => Ok(self.scy),
            0xFF43 => Ok(self.scx),
            0xFF44 => Ok(self.ly),
            0xFF45 => Ok(self.lyc),
            0xFF4a => Ok(self.wy),
            0xFF4b => Ok(self.wx),
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xFF40 => {
                self.lcdc = val;
                if val & 0x80 == 0 {
                    self.ly = 0;
                    self.current_mode = 0;
                    self.stat = (self.stat & 0b11111100) | self.current_mode;
                }
            }
            0xFF41 => self.stat = (val & 0b11111000) | 0b10000000,
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = 0,
            0xFF45 => self.lyc = val,
            0xFF4a => self.wy = val,
            0xFF4b => self.wx = val,
            _ => return Ok(()),
        }
        Ok(())
    }

    pub fn step(&mut self) {
        if self.lcdc & 0x80 == 0 {
            return;
        }
        match self.current_mode {
            2 => {
                // scan oem
                // 80 t-cycles
                if self.timer >= 80 {
                    self.current_mode = 3;
                    self.stat = (self.stat & 0b11111100) | self.current_mode;
                    self.timer = 0;
                }
            }
            3 => {
                // write pixels
                // 172-289 t-cycles
                if self.timer >= 172 {
                    self.current_mode = 0;
                    self.stat = (self.stat & 0b11111100) | self.current_mode;
                    self.timer = 0;
                }
            }
            0 => {
                // wait for next scan line
                // 87 - 204 t-cycles
                if self.timer >= 204 {
                    self.ly += 1;

                    self.current_mode = if self.ly >= 144 {
                        self.requesting_interrupt = true;
                        1
                    } else {
                        2
                    };
                    self.stat = (self.stat & 0b11111100) | self.current_mode;

                    self.timer = 0;
                }
            }
            1 => {
                // wait for next frame
                // 4560 t-cycles
                if self.timer >= 456 {
                    self.ly += 1;
                    if self.ly >= 154 {
                        self.ly = 0;
                        self.current_mode = 2;
                        self.stat = (self.stat & 0b11111100) | self.current_mode;
                    }
                    self.timer = 0;
                }
            }
            _ => unreachable!(),
        }
        self.timer += 1;

        self.stat = (self.stat & 0b11111011) | ((self.ly == self.lyc) as u8) << 2;
    }

    pub fn get_tile(&mut self, index: u16) -> [u8; 64] {
        let mut bytes: [u8; 16] = [0; 16];
        let mut tile: [u8; 64] = [0; 64];

        for i in 0..16 {
            bytes[i as usize] = self.vram[((index * 16) + i) as usize];
        }

        for line in (0..16).step_by(2) {
            let byte1 = bytes[line];
            let byte2 = bytes[line + 1];
            for bit in (0..8).rev() {
                let bit_mask = 1 << bit;
                let pixel = ((byte2 & bit_mask) >> bit) << 1 | (byte1 & bit_mask) >> bit;

                tile[(line * 4) + (7 - bit)] = pixel;
            }
        }

        tile
    }

    pub fn render_tile_banks(&mut self, buffer: &mut Vec<u32>) {
        for y in 0..24usize {
            for x in 0..16usize {
                let tile = self.get_tile((y * 16 + x) as u16);

                for py in 0..8usize {
                    for px in 0..8 {
                        let pixel_index = px + py * 8;
                        buffer[(x * 8 + y * (8 * 128)) + (px + py * 128)] =
                            self.palette[tile[pixel_index as usize] as usize];
                    }
                }
            }
        }
    }

    pub fn render_tile_maps(&mut self, buffer: &mut Vec<u32>) {
        for y in 0..64usize {
            for x in 0..32usize {
                let tile_index = self.vram[0x1800 + (0x20 * y) + x] as u16;

                let tile = self.get_tile(tile_index);

                for py in 0..8usize {
                    for px in 0..8usize {
                        let pixel_index = px + py * 8;

                        buffer[(x * 8 + y * (8 * 256)) + (px + py * 256)] =
                            self.palette[tile[pixel_index] as usize];
                    }
                }
            }
        }
    }

    pub(super) fn check_for_interrupt(&mut self) -> bool {
        if self.requesting_interrupt {
            self.requesting_interrupt = false;
            true
        } else {
            false
        }
    }
}
