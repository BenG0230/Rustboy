mod mbc;

use mbc::{Mbc, MbcTrait};
use std::{error::Error, fmt, fs};

#[derive(Debug)]
pub enum RomError {
    OutOfBounds(u16),
}

impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomError::OutOfBounds(addr) => write!(f, "rom -> Address {:#04X} out of bounds", addr),
        }
    }
}

pub struct Rom {
    mbc: Mbc,
}

impl Rom {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let data: Vec<u8> = fs::read(file_name)?;

        read_header(&data);

        Ok(Self {
            mbc: Mbc::new(data),
        })
    }

    // Call MBC's read function
    // see ./rom/mbc.rs and ./rom/mbc/*
    pub fn read_byte(&self, addr: u16) -> Result<u8, RomError> {
        self.mbc.read_byte(addr)
    }

    // Call MBC's write function
    // see ./rom/mbc.rs and ./rom/mbc/*
    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), RomError> {
        self.mbc.write_byte(addr, val)
    }
}

// Reads and prints information on the loaded ROM header
pub fn read_header(rom: &Vec<u8>) {
    println!(" -- NINTENDO LOGO (hopefully) -- ");
    let mut lines: Vec<String> = vec![String::new(); 8];
    for (i, byte) in rom[0x104..=0x133].iter().enumerate() {
        let mut idx = if i % 2 == 0 { 0 } else { 2 } + if i > 23 { 4 } else { 0 };
        for bit in (0..8).rev() {
            if bit == 3 {
                idx += 1;
            }
            lines[idx].push(if byte & (1 << bit) != 0 { '#' } else { '.' });
        }
    }
    for line in &lines {
        println!("{}", line);
    }

    println!("\n\n -- Game Meta-Data -- ");
    print!("Game Title: ");
    for byte in rom[0x134..=0x143].iter() {
        if *byte == 0 {
            println!();
            break;
        }
        print!("{}", *byte as char);
    }
    println!("Cartridge Type: {:#02X}", rom[0x147]);
    println!("ROM Size: {} KiB", 32 * (1 << rom[0x148]));
    print!("RAM Size: ");
    match rom[0x149] {
        0 => println!("No RAM!"),
        2 => println!("8 KiB"),
        3 => println!("32 KiB"),
        4 => println!("128 KiB"),
        5 => println!("64 KiB"),
        _ => println!("<Unknown Value>"),
    }
    print!("Destination: ");
    match rom[0x14A] {
        0 => println!("Japan"),
        1 => println!("Overseas"),
        _ => println!("<Unknown Value>"),
    }
    println!("Version Number: {}", rom[0x14C]);
    println!("\n\n");
}
