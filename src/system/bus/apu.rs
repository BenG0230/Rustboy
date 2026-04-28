mod square_channel;

use super::BusError;
use rand::Rng;
use square_channel::SquareChannel;

pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF10..=0xFF14 => self.ch1.read_byte(addr),
            0xFF16..=0xFF19 => self.ch2.read_byte(addr),
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF10..=0xFF14 => self.ch1.write_byte(addr, val),
            0xFF16..=0xFF19 => self.ch2.write_byte(addr, val),
            _ => Ok(()),
        }
    }

    pub fn step(&mut self) {
        self.ch1.tick();
        self.ch2.tick();
    }

    pub fn mix(&mut self) -> f32 {
        // Mix all channels together
        // Return single normalised value [-1.0,1.0]
        let sample1 = self.ch1.sample();
        let sample2 = self.ch2.sample();
        println!("{} {}", sample1, sample2);
        (sample1 + sample2) / 2.0
    }
}
