mod bus;
mod cpu;

use std::error::Error;

use bus::Bus;
use cpu::{Cpu, CpuError};

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
    pub fn run(&mut self) -> Result<(), CpuError> {
        //TODO: each loop call sub-systems steps
        //increment main system clock each loop according to cpu clock
        //Each sub-system "catches up" to main system clock
        loop {
            let steps = self.cpu.step(&mut self.bus)?;

            for _ in 0..steps {
                // Tick timers for each
                self.bus.tick_timers();
                if self.bus.check_timer_interrupt() {
                    // Set request bit for Timer interrupt HIGH
                    let interrupt_flag = self.bus.read_byte(0xFF0F)?;

                    self.bus.write_byte(0xFF0F, interrupt_flag | 0b00000100)?;
                }
            }
        }
    }
}
