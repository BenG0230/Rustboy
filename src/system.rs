mod bus;
mod cpu;

use std::error::Error;
use std::time::Duration;

use bus::{Bus, BusError};
use cpu::{Cpu, CpuError};

pub struct System {
    cpu: Cpu,
    bus: Bus,
}

impl System {
    // Create a new system with rom path/<rom_fname>
    pub fn new(rom_fname: &str) -> Result<Self, Box<dyn Error>> {
        let bus = Bus::new(rom_fname)?;

        Ok(Self {
            cpu: Cpu::new(),
            bus,
        })
    }

    pub fn render_tile_banks(&mut self, buffer: &mut Vec<u32>) {
        self.bus.render_tile_banks(buffer);
    }

    pub fn render_tile_maps(&mut self, buffer: &mut Vec<u32>) {
        self.bus.render_tile_maps(buffer);
    }

    pub fn get_frame_buffer(&mut self) -> &mut Vec<u32> {
        self.bus.get_frame_buffer()
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

                self.bus.write_byte(0xFF0F, interrupt_flag | 0b100)?;
            }

            self.bus.step_ppu();
            if self.bus.check_vblank_interrupt() {
                let interrupt_flag = self.bus.read_byte(0xFF0F)?;
                self.bus.write_byte(0xFF0F, interrupt_flag | 0b1)?;
            }
        }

        Ok(())
    }
}
