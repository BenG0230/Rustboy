mod bus;
mod cpu;

use std::error::Error;

use bus::Bus;
use cpu::Cpu;

pub struct System {
    cpu: Cpu,
    bus: Bus,
}

impl System {
    pub fn new(rom_fname: &str) -> Result<Self, Box<dyn Error>> {
        let mut bus = Bus::new();
        bus.load_rom(rom_fname)?;

        Ok(Self {
            cpu: Cpu::new(),
            bus,
        })
    }

    pub fn run(&mut self) {
        //TODO: each loop call sub-systems steps
        //increment main system clock each loop according to cpu clock
        //Each sub-system "catches up" to main system clock
        loop {
            let steps = self.cpu.step(&mut self.bus);
        }
    }
}
