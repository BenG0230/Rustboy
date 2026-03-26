use crate::system::bus::rom::RomError;

pub struct Mbc3 {
    // --- Memory ---
    rom: Vec<u8>,
    ram: Vec<u8>,
    rtc_regs: [u8; 5],
    // --- Registers ---
    rom_bank: u8,
    ram_bank: u8,
    // --- Flags ---
    ram_enable: bool,
}

impl Mbc3 {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let ram_size = match rom_data[0x149] {
            // Dynamically set RAM size based on byte 0x149 in ROM
            1 => 1024 * 2,
            2 => 1024 * 8,
            3 => 1024 * 32,
            4 => 1024 * 128,
            5 => 1024 * 64,
            _ => 0,
        };

        Self {
            rom: rom_data,
            ram: vec![0; ram_size],
            rtc_regs: [0; 5],
            rom_bank: 1,
            ram_bank: 0,
            ram_enable: false,
        }
    }
}

impl super::MbcTrait for Mbc3 {
    fn read_byte(&self, addr: u16) -> Result<u8, RomError> {
        match addr {
            0x0000..=0x3FFF => Ok(self.rom[addr as usize]),
            0x4000..=0x7FFF => {
                let max_bank = self.rom.len() / 0x4000;

                let bank_number = (self.rom_bank as usize) % max_bank;

                let banked_addr = bank_number * 0x4000 + (addr as usize - 0x4000);

                Ok(self.rom[banked_addr])
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    match self.ram_bank {
                        0..=0x7 => {
                            // Ram
                            let max_bank = self.ram.len() / 0x2000;

                            let bank_number = (self.ram_bank as usize) % max_bank;

                            let banked_addr = bank_number * 0x2000 + (addr as usize - 0xA000);

                            Ok(self.ram[banked_addr])
                        }
                        _ => Ok(0xFF),
                    }
                } else {
                    Ok(0xFF)
                }
            }
            _ => Ok(0xFF),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), RomError> {
        match addr {
            0x0000..=0x1FFF => {
                if val == 0xA {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            }
            0x2000..=0x3FFF => {
                self.rom_bank = val & 0b1111111;

                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }
            0x4000..=0x5FFF => {
                self.ram_bank = val & 0b1111;

                if self.ram_bank == 0 {
                    self.ram_bank = 1;
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    match self.ram_bank {
                        0..=0x7 => {
                            // Ram
                            let banked_addr =
                                (self.ram_bank as usize) * 0x2000 + (addr as usize - 0xA000);

                            if banked_addr < self.ram.len() {
                                self.ram[banked_addr] = val;
                            }
                        }
                        _ => {}
                    }
                }
            }
            0x6000..=0x7FFF => {
                // :3
            }
            _ => return Err(RomError::OutOfBounds(addr)),
        }
        Ok(())
    }
}
