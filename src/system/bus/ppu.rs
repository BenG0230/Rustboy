mod ppumemory;

use super::BusError;
use ppumemory::PpuMemory;

pub struct Ppu {
    // --- Display stuff ---
    buffer: Vec<u32>,
    palette: [u32; 4],

    // VRAM
    memory: PpuMemory,

    // Emulator stuffs
    timer: u32,

    // flags
    req_vblank_interrupt: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            // --- Display stuff ---
            buffer: vec![0; 160 * 144],
            palette: [0x9a9e3f, 0x496b22, 0x0e450b, 0x1b2a09],

            // --- Vram ---
            memory: PpuMemory::new(),

            // --- Emulation stuff ---
            timer: 0,

            // --- Flags ---
            req_vblank_interrupt: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        self.memory.read_byte(addr)
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        self.memory.write_byte(addr, val)
    }

    pub fn step(&mut self) {
        // stop if ppu is disabled
        if self.memory.lcdc & 0x80 == 0 {
            return;
        }

        // Draw line if moving from mode 2 -> 3
        // timer = 80
        if self.timer == 80 && self.memory.ly < 144 {
            self.draw_scan_line();
        }

        // Check for STAT interrupt

        self.timer += 1;
        if self.timer >= 456 {
            self.timer = 0;
            self.memory.ly += 1;

            if self.memory.ly == 144 {
                self.req_vblank_interrupt = true;
            }

            if self.memory.ly == 154 {
                self.memory.ly = 0;
            }
        }
    }

    fn draw_scan_line(&mut self) {
        let line = self.memory.ly as usize;

        // render background
        for dx in 0..160 {
            // Pixel positions in background
            let x = (self.memory.scx as usize + dx) % 256;
            let y = (self.memory.scy as usize + line) % 256;

            // Tile positions in background
            let tile_x = x / 8;
            let tile_y = y / 8;

            // Tile pixel position
            let pixel_x = x % 8;
            let pixel_y = y % 8;

            let map_bank_offset = if self.memory.lcdc & 0b00001000 > 0 {
                0x1C00
            } else {
                0x1800
            };
            let mut tile_index =
                self.memory.vram[map_bank_offset + tile_x + (0x20 * tile_y)] as usize;
            if self.memory.lcdc & 0b10000 == 0 {
                if tile_index < 0x80 {
                    tile_index += 0x100;
                }
            }

            let tile_addr = (tile_index * 16) + (2 * pixel_y);

            let bit1 = (self.memory.vram[tile_addr] & (1 << (7 - pixel_x))) >> (7 - pixel_x);
            let bit2 = (self.memory.vram[tile_addr + 1] & (1 << (7 - pixel_x))) >> (7 - pixel_x);
            let palette_index = bit2 << 1 | bit1;

            let colour_index = self.memory.bgp[palette_index as usize];
            let colour = self.palette[colour_index as usize];

            self.buffer[dx + line * 160] = colour;
        }

        // render window
        // render sprites (maybe)
    }

    pub(super) fn get_frame_buffer(&mut self) -> &mut Vec<u32> {
        return &mut self.buffer;
    }

    pub(super) fn check_for_vblankinterrupt(&mut self) -> bool {
        if self.req_vblank_interrupt {
            self.req_vblank_interrupt = false;
            true
        } else {
            false
        }
    }

    // ### Debug Rendering ###

    pub fn get_tile(&mut self, index: u16) -> [u8; 64] {
        let mut bytes: [u8; 16] = [0; 16];
        let mut tile: [u8; 64] = [0; 64];

        for i in 0..16usize {
            bytes[i as usize] = self.memory.vram[(index as usize * 16) + i];
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
                        let colour_indice = self.memory.bgp[tile[pixel_index] as usize];
                        let colour = self.palette[colour_indice as usize];

                        buffer[(x * 8 + y * (8 * 128)) + (px + py * 128)] = colour;
                    }
                }
            }
        }
    }

    pub fn render_tile_maps(&mut self, buffer: &mut Vec<u32>) {
        for y in 0..64usize {
            for x in 0..32usize {
                let mut tile_index = self.memory.vram[0x1800 + (0x20 * y) + x] as u16;

                if self.memory.lcdc & 0b10000 == 0 {
                    if tile_index < 0x80 {
                        tile_index += 0x100;
                    }
                }

                let tile = self.get_tile(tile_index);

                for py in 0..8usize {
                    for px in 0..8usize {
                        let pixel_index = px + py * 8;
                        let colour_indice = self.memory.bgp[tile[pixel_index] as usize];
                        let colour = self.palette[colour_indice as usize];

                        buffer[(x * 8 + y * (8 * 256)) + (px + py * 256)] = colour;
                    }
                }
            }
        }
    }
}
