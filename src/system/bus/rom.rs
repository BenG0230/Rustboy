use std::{error::Error, fmt, fs};

#[derive(Debug)]
pub enum RomError {
    WriteToRom(u16),
    OutOfBounds(u16),
}

impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomError::WriteToRom(addr) => write!(f, "Cannot Write to ROM at address {:#04X}", addr),
            RomError::OutOfBounds(addr) => write!(f, "rom -> Address {:#04X} out of bounds", addr),
        }
    }
}

pub struct Rom {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Rom {
    pub fn new() -> Self {
        Self {
            rom: Vec::new(),
            ram: Vec::new(),
        }
    }

    // Loads ROM from file and sets correct size for RAM
    pub fn load_rom(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        // Load ROM from file
        let data: Vec<u8> = fs::read(file_name)?;

        self.rom = data;
        let ram_size = match self.rom[0x149] {
            // Dynamically set RAM size based on byte 0x149 in ROM
            2 => 1024 * 8,
            3 => 1024 * 32,
            4 => 1024 * 128,
            5 => 1024 * 64,
            _ => 0,
        };
        self.ram = vec![0; ram_size];

        Ok(())
    }

    // Read a single byte from ROM or RAM
    // Maps address 0x0000-7FFFF -> ROM
    //              0xA000-BFFFF -> RAM
    pub fn read_byte(&self, addr: u16) -> Result<u8, RomError> {
        match addr {
            0x0000..=0x7FFF => self
                .rom
                .get(addr as usize)
                .copied()
                .ok_or(RomError::OutOfBounds(addr)),
            0xA000..=0xBFFF => self
                .ram
                .get((addr - 0xA000) as usize)
                .copied()
                .ok_or(RomError::OutOfBounds(addr)),
            _ => Err(RomError::OutOfBounds(addr)),
        }
    }

    // Read a single byte from RAM
    // Maps address 0x0000-7FFFF -> ROM (Rom write error)
    //              0xA000-BFFFF -> RAM
    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), RomError> {
        match addr {
            0x0000..=0x7FFF => Err(RomError::WriteToRom(addr)),
            0xA000..=0xBFFF => {
                let ram_addr = (addr - 0xA000) as usize;
                if ram_addr >= self.ram.len() {
                    Err(RomError::OutOfBounds(addr))
                } else {
                    self.ram[ram_addr] = val;
                    Ok(())
                }
            }
            _ => Err(RomError::OutOfBounds(addr)),
        }
    }

    // Reads and prints information on the loaded ROM header
    pub fn read_header(&self) {
        println!(" -- NINTENDO LOGO (hopefully) -- ");
        let mut lines: Vec<String> = vec![String::new(); 8];
        for (i, byte) in self.rom[0x104..=0x133].iter().enumerate() {
            let mut idx = if i % 2 == 0 { 0 } else { 2 } + if i > 23 { 4 } else { 0 };
            for bit in (0..8).rev() {
                if bit == 3 {
                    idx += 1;
                }
                lines[idx].push(if byte & (1 << bit) != 0 { 'X' } else { '.' });
            }
        }
        for line in &lines {
            println!("{}", line);
        }

        println!("\n\n -- Game Meta-Data -- ");
        print!("Game Title: ");
        for byte in self.rom[0x134..=0x143].iter() {
            if *byte == 0 {
                println!();
                break;
            }
            print!("{}", *byte as char);
        }
        println!("Cartridge Type: {:#02X}", self.rom[0x147]);
        println!("ROM Size: {} KiB", 32 * (1 << self.rom[0x148]));
        print!("RAM Size: ");
        match self.rom[0x149] {
            0 => println!("No RAM!"),
            2 => println!("8 KiB"),
            3 => println!("32 KiB"),
            4 => println!("128 KiB"),
            5 => println!("64 KiB"),
            _ => println!("<Unknown Value>"),
        }
        print!("Destination: ");
        match self.rom[0x14A] {
            0 => println!("Japan"),
            1 => println!("Overseas"),
            _ => println!("<Unknown Value>"),
        }
        println!("Version Number: {}", self.rom[0x14C]);
    }
}
