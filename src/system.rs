mod bus;
mod cpu;
mod ppu;

use std::error::Error;
use std::time::Duration;

use bus::{Bus, BusError};
use cpu::{Cpu, CpuError};
use ppu::Ppu;

pub struct System {
    cpu: Cpu,
    bus: Bus,
    pub ppu: Ppu,
}

impl System {
    // Create a new system with rom path/<rom_fname>
    pub fn new(rom_fname: &str) -> Result<Self, Box<dyn Error>> {
        let bus = Bus::new(rom_fname)?;

        Ok(Self {
            cpu: Cpu::new(),
            bus,
            ppu: Ppu::new(),
        })
    }

    pub fn render_tile_banks(&mut self, buffer: &mut Vec<u32>) {
        self.ppu.render_tile_banks(&mut self.bus, buffer);
    }

    // Run next instruction
    // Returns number of t-cycles taken
    pub fn step_cpu(&mut self) -> Result<u8, CpuError> {
        self.cpu.step(&mut self.bus)
    }

    // Tick subSystems by number of t-cycles
    pub fn tick_subsystems(&mut self, steps: u8) -> Result<(), BusError> {
        for _ in 0..steps {
            // Tick timers for each
            self.bus.tick_timers();
            if self.bus.check_timer_interrupt() {
                // Set request bit for Timer interrupt HIGH
                let interrupt_flag = self.bus.read_byte(0xFF0F)?;

                self.bus.write_byte(0xFF0F, interrupt_flag | 0b00000100)?;
            }
        }

        Ok(())
    }
}
