mod bus;
mod cpu;

use std::time::Duration;
use std::{error::Error, iter::Cycle};

use bus::{Bus, BusError};
use cpu::{Cpu, CpuError};

pub struct System {
    cpu: Cpu,
    bus: Bus,

    pub vblank: bool,
}

impl System {
    // Create a new system with rom path/<rom_fname>
    pub fn new(rom_fname: &str) -> Result<Self, Box<dyn Error>> {
        let bus = Bus::new(rom_fname)?;

        Ok(Self {
            cpu: Cpu::new(),
            bus,
            vblank: false,
        })
    }

    pub fn copy_frame_buffer(&mut self, frame: &mut [u8]) {
        frame.copy_from_slice(self.bus.get_frame_buffer());
    }

    pub(super) fn change_key(&mut self, button_index: usize, val: bool) {
        self.bus.change_key(button_index, val);
    }

    pub fn run_until_vblank(&mut self) -> u32 {
        let mut cycles_elapsed = 0;
        while !self.vblank {
            let steps = self
                .step_cpu()
                .unwrap_or_else(|e| panic!("Failed to step CPU: {e}"));

            self.tick_subsystems(steps)
                .unwrap_or_else(|e| panic!("Failed to tick subSystems: {e}"));
            cycles_elapsed += 1;
        }

        self.vblank = false;
        cycles_elapsed
    }

    // Run next instruction
    // Returns number of t-cycles taken
    pub fn step_cpu(&mut self) -> Result<u8, CpuError> {
        self.cpu.step(&mut self.bus)
    }

    // Tick sub Systems by number of t-cycles
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
                self.vblank = true;
                let interrupt_flag = self.bus.read_byte(0xFF0F)?;
                self.bus.write_byte(0xFF0F, interrupt_flag | 0b1)?;
            }
            if self.bus.check_stat_interrupt() {
                let interrupt_flag = self.bus.read_byte(0xFF0F)?;
                self.bus.write_byte(0xFF0F, interrupt_flag | 0b10)?;
            }
        }

        Ok(())
    }
}
