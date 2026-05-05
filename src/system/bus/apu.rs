mod square_channel;

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use super::BusError;
use rand::{Rng, seq::index::sample};
use square_channel::SquareChannel;

pub struct Apu {
    enabled: bool,
    ch1: SquareChannel,
    ch2: SquareChannel,
    sample_timer: f64,
    pub buffer: Arc<Mutex<VecDeque<f32>>>,
    accumulator: u8,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            enabled: true,
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            sample_timer: 0.0,
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            accumulator: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF26 => {
                let mut output = 0b01110000;
                output |= (self.enabled as u8) << 7;
                Ok(output)
            }
            0xFF10..=0xFF14 => self.ch1.read_byte(addr),
            0xFF16..=0xFF19 => self.ch2.read_byte(addr),
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF26 => self.enabled = (val & 0b10000000) > 0,
            0xFF10..=0xFF14 => self.ch1.write_byte(addr, val)?,
            0xFF16..=0xFF19 => self.ch2.write_byte(addr, val)?,
            _ => {}
        }
        Ok(())
    }

    pub fn step(&mut self) {
        if !self.enabled {
            return;
        }
        // Called every t-cycle
        // keep track of mix timings (shown below)
        // tick channels for state updates

        let master_clock_speed = 4194304.0;
        let sample_rate = 44100.0;
        let cycles_per_sample = master_clock_speed / sample_rate;

        self.sample_timer += 1 as f64;

        if self.sample_timer >= cycles_per_sample {
            self.sample_timer -= cycles_per_sample;
            self.mix();
        }

        self.accumulator += 1;
        if self.accumulator >= 4 {
            self.ch1.tick();
            self.ch2.tick();
            self.accumulator = 0;
        }
    }

    pub fn mix(&self) {
        if !self.enabled {
            return;
        }
        // called at 44.1kHz (rodio source sample rate)
        // 4.194304MHz / 44.1kHz ~= 95.11
        // so called every 95.11 t-cycles
        // add samples to buffer for rodio to consume when needed at 44.1kHz
        //
        // add together all channels samples and normalise
        //
        let ch1_sample = self.ch1.sample();
        let ch2_sample = self.ch2.sample();

        let sample = (ch1_sample + ch2_sample) / 2.0;

        self.buffer.lock().unwrap().push_back(sample);
    }
}
