mod ppumemory;

use super::BusError;
use ppumemory::PpuMemory;

pub struct Ppu {
    // --- Display stuff ---
    buffer: Vec<u8>,
    palette: [(u8, u8, u8); 4],

    // VRAM
    memory: PpuMemory,

    // Emulator stuffs
    timer: u32,

    // flags
    req_stat_interrupt: bool,
    req_vblank_interrupt: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            // --- Display stuff ---
            buffer: vec![0xFF; 160 * 144 * 4],
            palette: [
                (0x9a, 0x9e, 0x3f),
                (0x49, 0x6b, 0x22),
                (0x0e, 0x45, 0x0b),
                (0x1b, 0x2a, 0x09),
            ],

            // --- Vram ---
            memory: PpuMemory::new(),

            // --- Emulation stuff ---
            timer: 0,

            // --- Flags ---
            req_stat_interrupt: false,
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

        self.timer += 1;

        let mode = if self.memory.ly < 144 {
            match self.timer {
                0 => {
                    if self.memory.stat & 0b00100000 > 0 {
                        self.req_stat_interrupt = true;
                    }
                    2
                }
                80 => {
                    self.draw_scan_line();
                    3
                }
                252 => {
                    if self.memory.stat & 0b00001000 > 0 {
                        self.req_stat_interrupt = true;
                    }
                    0
                }
                _ => self.memory.stat & 0b11,
            }
        } else {
            if self.memory.stat & 0b11 != 1 && self.memory.stat & 0b00010000 > 0 {
                self.req_stat_interrupt = true;
            }
            1
        };

        self.memory.stat = (self.memory.stat & 0b11111100) | mode;

        // Check for STAT interrupt

        if self.timer >= 456 {
            self.timer = 0;
            self.memory.ly += 1;

            if self.memory.ly == 144 {
                self.req_vblank_interrupt = true;
            }

            if self.memory.ly == 154 {
                self.memory.ly = 0;
            }

            if self.memory.ly == self.memory.lyc {
                if self.memory.stat & 0b100 == 0 && self.memory.stat & 0b01000000 > 0 {
                    self.req_stat_interrupt = true;
                }

                self.memory.stat = self.memory.stat | 0b100;
            } else {
                self.memory.stat = self.memory.stat & 0b11111011;
            }
        }
    }

    fn draw_scan_line(&mut self) {
        let line = self.memory.ly as usize;

        let window_shown =
            self.memory.lcdc & 0b00100000 > 0 && self.memory.wx < 167 && self.memory.wy < 144;

        // Scan OAM

        let ob_height = if self.memory.lcdc & 0b100 == 0 { 8 } else { 16 };
        let mut num_ob_in_line = 0;
        let mut ob_in_line: [i8; 10] = [-1; 10];

        if self.memory.lcdc & 0b10 > 0 {
            for i in 0..40 {
                let ob_y = self.memory.oam[i * 4];

                if line + 16 >= ob_y as usize
                    && line + 16 < ob_y as usize + ob_height
                    && num_ob_in_line < 10
                {
                    ob_in_line[num_ob_in_line] = i as i8;
                    num_ob_in_line += 1;
                }
            }
        }

        for dx in (0..160usize).rev() {
            // Pixel position in the background/window
            let x;
            let y;
            // Address offset for tile map
            let map_bank_offset;

            if !window_shown || line < self.memory.wy as usize || dx + 7 < self.memory.wx as usize {
                // Window is not drawn over background
                x = (self.memory.scx as usize + dx) % 256;
                y = (self.memory.scy as usize + line) % 256;

                map_bank_offset = if self.memory.lcdc & 0b00001000 > 0 {
                    0x1C00
                } else {
                    0x1800
                };
            } else {
                // Window is drawn over background
                x = dx + 7 - self.memory.wx as usize;
                y = line - self.memory.wy as usize;

                map_bank_offset = if self.memory.lcdc & 0b01000000 > 0 {
                    0x1C00
                } else {
                    0x1800
                };
            }

            // Tile position in background/window
            let tile_x = x / 8;
            let tile_y = y / 8;

            // Pixel position in tile
            let pixel_x = x % 8;
            let pixel_y = y % 8;

            // Address of tile index in background map
            let tile_index_addr = map_bank_offset + tile_x + (0x20 * tile_y);

            // Index of tile
            let mut tile_index = self.memory.vram[tile_index_addr] as usize;
            if self.memory.lcdc & 0b10000 == 0 && tile_index < 0x80 {
                // if in addressing mode 0
                tile_index += 0x100;
            }

            // address location of tile data
            let tile_addr = (tile_index * 16) + (2 * pixel_y);

            // build 2bbp from 2 tile bytes
            let bit1 = (self.memory.vram[tile_addr] >> (7 - pixel_x)) & 1;
            let bit2 = (self.memory.vram[tile_addr + 1] >> (7 - pixel_x)) & 1;
            let palette_index = bit2 << 1 | bit1;

            let colour_index = self.memory.bgp[palette_index as usize];
            let colour = self.palette[colour_index as usize];

            // draw to screen
            let pixel_index = (dx + line * 160) * 4;
            if self.memory.lcdc & 1 == 1 {
                self.buffer[pixel_index] = colour.0;
                self.buffer[pixel_index + 1] = colour.1;
                self.buffer[pixel_index + 2] = colour.2;
                self.buffer[pixel_index + 3] = 0xFF;
            } else {
                self.buffer[pixel_index] = 0x00;
                self.buffer[pixel_index + 1] = 0x00;
                self.buffer[pixel_index + 2] = 0x00;
                self.buffer[pixel_index + 3] = 0xFF;
            }

            // For every object on this line
            for ob_index in ob_in_line {
                if ob_index == -1 {
                    break;
                }

                // Object data
                let ob_y = self.memory.oam[ob_index as usize * 4];
                let ob_x = self.memory.oam[ob_index as usize * 4 + 1];
                let flags = self.memory.oam[ob_index as usize * 4 + 3];

                if dx + 8 >= ob_x as usize && dx < ob_x as usize {
                    // Dont draw pixel if BG/Window priority and BG/Window palette index 1-3
                    if flags & 0b10000000 > 0 && palette_index != 0 {
                        continue;
                    }

                    // X position in tile
                    // Flipped if Xflip flag set
                    let pixel_x = if flags & 0b100000 == 0 {
                        dx as u8 + 8 - ob_x
                    } else {
                        7 - (dx as u8 + 8 - ob_x)
                    };

                    // Y position in tile
                    // Flipped if Yflip flag set
                    let pixel_y = if flags & 0b1000000 == 0 {
                        line as u8 + 16 - ob_y
                    } else {
                        (ob_height as u8 - 1) - (line as u8 + 16 - ob_y)
                    };

                    let tile_index = self.memory.oam[ob_index as usize * 4 + 2] as usize;

                    let tile_addr = tile_index * 16 + (pixel_y as usize * 2);

                    // build 2bbp from 2 tile bytes
                    let bit1 = (self.memory.vram[tile_addr] >> (7 - pixel_x)) & 1;
                    let bit2 = (self.memory.vram[tile_addr + 1] >> (7 - pixel_x)) & 1;
                    let ob_palette_index = bit2 << 1 | bit1;

                    // Use object pallette 0/1 based on palette flag
                    let colour_index = if flags & 0b10000 == 0 {
                        self.memory.obp0[ob_palette_index as usize]
                    } else {
                        self.memory.obp1[ob_palette_index as usize]
                    };

                    // Palette index 0 = transparent
                    if ob_palette_index != 0 {
                        let colour = self.palette[colour_index as usize];
                        let pixel_index = (dx + line * 160) * 4;
                        self.buffer[pixel_index] = colour.0;
                        self.buffer[pixel_index + 1] = colour.1;
                        self.buffer[pixel_index + 2] = colour.2;
                        self.buffer[pixel_index + 3] = 0xFF;
                    }
                }
            }
        }
    }

    pub(super) fn get_frame_buffer(&mut self) -> &mut Vec<u8> {
        return &mut self.buffer;
    }

    pub(super) fn check_for_statinterrupt(&mut self) -> bool {
        if self.req_stat_interrupt {
            self.req_stat_interrupt = false;
            true
        } else {
            false
        }
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

    // pub fn get_tile(&mut self, index: u16) -> [u8; 64] {
    //     let mut bytes: [u8; 16] = [0; 16];
    //     let mut tile: [u8; 64] = [0; 64];
    //
    //     for i in 0..16usize {
    //         bytes[i as usize] = self.memory.vram[(index as usize * 16) + i];
    //     }
    //
    //     for line in (0..16).step_by(2) {
    //         let byte1 = bytes[line];
    //         let byte2 = bytes[line + 1];
    //         for bit in (0..8).rev() {
    //             let bit_mask = 1 << bit;
    //             let pixel = ((byte2 & bit_mask) >> bit) << 1 | (byte1 & bit_mask) >> bit;
    //
    //             tile[(line * 4) + (7 - bit)] = pixel;
    //         }
    //     }
    //
    //     tile
    // }
    //
    // pub fn render_tile_banks(&mut self, buffer: &mut Vec<u32>) {
    //     for y in 0..24usize {
    //         for x in 0..16usize {
    //             let tile = self.get_tile((y * 16 + x) as u16);
    //
    //             for py in 0..8usize {
    //                 for px in 0..8 {
    //                     let pixel_index = px + py * 8;
    //                     let colour_indice = self.memory.bgp[tile[pixel_index] as usize];
    //                     let colour = self.palette[colour_indice as usize];
    //
    //                     buffer[(x * 8 + y * (8 * 128)) + (px + py * 128)] = colour;
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // pub fn render_tile_maps(&mut self, buffer: &mut Vec<u32>) {
    //     for y in 0..64usize {
    //         for x in 0..32usize {
    //             let mut tile_index = self.memory.vram[0x1800 + (0x20 * y) + x] as u16;
    //
    //             if self.memory.lcdc & 0b10000 == 0 {
    //                 if tile_index < 0x80 {
    //                     tile_index += 0x100;
    //                 }
    //             }
    //
    //             let tile = self.get_tile(tile_index);
    //
    //             for py in 0..8usize {
    //                 for px in 0..8usize {
    //                     let pixel_index = px + py * 8;
    //                     let colour_indice = self.memory.bgp[tile[pixel_index] as usize];
    //                     let colour = self.palette[colour_indice as usize];
    //
    //                     if y >= 32 {
    //                         buffer[(x * 8 + y * (8 * 256)) + (px + (py + 1) * 256)] = colour;
    //                     } else {
    //                         buffer[(x * 8 + y * (8 * 256)) + (px + py * 256)] = colour;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // scxy border box
    //     let scy_offset = if self.memory.lcdc & 0b1000 == 0 {
    //         0
    //     } else {
    //         257
    //     };
    //
    //     for i in (self.memory.scx as usize)..=(self.memory.scx as usize + 160) {
    //         let x = i % 256;
    //
    //         buffer[x + 256 * (self.memory.scy as usize + scy_offset)] = 0xFF0000;
    //         buffer[x + 256 * (((self.memory.scy as usize + 144) % 256) + scy_offset)] = 0xFF0000;
    //     }
    //
    //     for i in (self.memory.scy as usize)..=(self.memory.scy as usize + 144) {
    //         let y = i % 256 + scy_offset;
    //
    //         buffer[y * 256 + self.memory.scx as usize] = 0xFF0000;
    //         buffer[y * 256 + self.memory.scx as usize + 160] = 0xFF0000;
    //     }
    //
    //     // window border box
    //     if self.memory.lcdc & 0b00100000 > 0 && self.memory.wx < 167 && self.memory.wy < 144 {
    //         let wy_offset = if self.memory.lcdc & 0b01000000 == 0 {
    //             0
    //         } else {
    //             257
    //         };
    //
    //         for i in 0..=(167 - self.memory.wx as usize) {
    //             let x = i % 256;
    //
    //             buffer[x + 256 * wy_offset] = 0x0000FF;
    //             buffer[x + 256 * (wy_offset + (144 - self.memory.wy as usize))] = 0x0000FF;
    //         }
    //
    //         for i in 0..=(144 - self.memory.wy as usize) {
    //             let y = i % 256 + wy_offset;
    //
    //             buffer[y * 256] = 0x0000FF;
    //             buffer[y * 256 + (167 - self.memory.wx as usize)] = 0x0000FF;
    //         }
    //     }
    // }
}
