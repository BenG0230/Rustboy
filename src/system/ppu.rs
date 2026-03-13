use super::bus::{Bus, BusError};

pub struct Ppu {
    palette: [u32; 4],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            // #9a9e3f
            // #496b22
            // #0e450b
            // #1b2a09
            palette: [0x9a9e3f, 0x496b22, 0x0e450b, 0x1b2a09],
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {}

    pub fn get_tile(&mut self, bus: &mut Bus, index: u16) -> [u8; 64] {
        let mut bytes: [u8; 16] = [0; 16];
        let mut tile: [u8; 64] = [0; 64];

        for i in 0..16u16 {
            bytes[i as usize] = bus
                .read_byte(0x8000 + (index * 16) + i)
                .unwrap_or_else(|e| panic!("AHHHH {}", e));
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

    pub fn render_tile_banks(&mut self, bus: &mut Bus, buffer: &mut Vec<u32>) {
        for y in 0..24 {
            for x in 0..16 {
                let tile = self.get_tile(bus, y * 16 + x);

                buffer[(x * 8 + y * 1024) as usize] = 0xff0000;

                for py in 0..8 {
                    for px in 0..8 {
                        let pixel_index = px + py * 8;
                        buffer[((x * 8 + y * 1024) + (px + py * 128)) as usize] =
                            self.palette[tile[pixel_index as usize] as usize];
                    }
                }
            }
        }
    }
}
