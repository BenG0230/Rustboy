use crate::system::bus::rom::RomError;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    reg1: u8, // 5 bit register - denotes lower 5 bits of ROM bank number
    reg2: u8, // 2 bit register - denotes upper 2 bits of ROM bank number
    // and Static ROM bank number (Mode 1)
    // and RAM bank number (Mode 1)
    banking_mode: u8, // 0 for mode 0
    // 1 for mode 1
    ram_enable: bool, // Is RAM enabled
}

impl Mbc1 {
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
            reg1: 1,
            reg2: 0,
            ram_enable: false,
            banking_mode: 0,
        }
    }
}

impl super::MbcTrait for Mbc1 {
    fn read_byte(&self, addr: u16) -> Result<u8, RomError> {
        match addr {
            0x0000..=0x3FFF => {
                let max_bank = self.rom.len() / 0x4000;

                let bank_number = if self.banking_mode == 0 {
                    0
                } else {
                    ((self.reg2 as usize) << 5) % max_bank
                };

                let banked_addr = bank_number * 0x4000 + (addr as usize);

                self.rom
                    .get(banked_addr)
                    .copied()
                    .ok_or(RomError::OutOfBounds(addr))
            }
            0x4000..=0x7FFF => {
                let max_bank = self.rom.len() / 0x4000;

                let bank_number = (((self.reg2 as usize) << 5) | (self.reg1 as usize)) % max_bank;

                let banked_addr = bank_number * 0x4000 + (addr as usize - 0x4000);

                // println!(
                //     "{:#06X}, {:#04b}, {:#07b}-> {:#06X}",
                //     addr, self.reg2, self.reg1, banked_addr
                // );

                self.rom
                    .get(banked_addr)
                    .copied()
                    .ok_or(RomError::OutOfBounds(addr))
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let max_bank = self.ram.len() / 0x2000;

                    let bank_number = if self.banking_mode == 0 {
                        0
                    } else {
                        (self.reg2 as usize) % max_bank
                    };

                    let banked_addr = bank_number * 0x2000 + (addr as usize - 0xA000);

                    self.ram
                        .get(banked_addr)
                        .copied()
                        .ok_or(RomError::OutOfBounds(addr))
                } else {
                    Ok(0xFF)
                }
            }
            _ => Err(RomError::OutOfBounds(addr)),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), RomError> {
        match addr {
            0x0000..=0x1FFF => {
                // enable RAM if 0x#A
                if val & 0xF == 0xA {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
                Ok(())
            }
            0x2000..=0x3FFF => {
                self.reg1 = val & 0b11111;

                if self.reg1 == 0 {
                    self.reg1 = 1;
                }

                Ok(())
            }
            0x4000..=0x5FFF => {
                self.reg2 = val & 0b11;

                Ok(())
            }
            0x6000..=0x7FFF => {
                // Banking mode 1 or 0 based on bit 0
                self.banking_mode = val & 1;

                Ok(())
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let max_bank = self.ram.len() / 0x2000;

                    let bank_number = if self.banking_mode == 0 {
                        0
                    } else {
                        (self.reg2 as usize) % max_bank
                    };

                    let banked_addr = bank_number * 0x2000 + (addr as usize - 0xA000);

                    if banked_addr >= self.ram.len() {
                        Err(RomError::OutOfBounds(addr))
                    } else {
                        self.ram[banked_addr] = val;
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
            _ => Err(RomError::OutOfBounds(addr)),
        }
    }
}
