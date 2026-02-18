mod bus;
mod cpu;

use std::{error::Error, time::Duration};

use bus::Bus;
use cpu::Cpu;

pub struct System {
    cpu: Cpu,
    bus: Bus,
}

impl System {
    // Create a new system with rom path/<rom_fname>
    pub fn new(rom_fname: &str) -> Result<Self, Box<dyn Error>> {
        let mut bus = Bus::new();
        bus.load_rom(rom_fname)?;

        Ok(Self {
            cpu: Cpu::new(),
            bus,
        })
    }

    // Run the loaded rom
    pub fn run(&mut self) {
        //TODO: each loop call sub-systems steps
        //increment main system clock each loop according to cpu clock
        //Each sub-system "catches up" to main system clock
        loop {
            // std::thread::sleep(Duration::from_nanos(500));
            let steps = self.cpu.step(&mut self.bus);
        }
    }
}
