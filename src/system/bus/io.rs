mod timer;

use crate::system::bus::BusError;
use std::io::Write;
use std::io::stdout;
use timer::Timers;

pub struct Io {
    io_regs: [u8; 128],
    timers: Timers,
    // right left up down a b select start
    joypad: [bool; 8],
}

impl Io {
    pub fn new() -> Self {
        Self {
            io_regs: [0xFF; 128],
            timers: Timers::new(),
            joypad: [false; 8],
        }
    }

    pub(super) fn read_reg(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF00 => {
                let mut reg_val = 0xFF;

                if self.io_regs[0] & 0b00010000 == 0 {
                    for i in 0..3 {
                        reg_val &= (!self.joypad[i] as u8) << i;
                    }
                }

                if self.io_regs[0] & 0b00100000 == 0 {
                    for i in 0..3 {
                        reg_val &= (!self.joypad[i + 4] as u8) << i;
                    }
                }

                println!("{:08b}", reg_val);
                Ok(reg_val)
            }
            0xFF04..=0xFF07 => self.timers.read_reg(addr),
            _ => Ok(self.io_regs[(addr - 0xFF00) as usize]),
        }
    }

    pub(super) fn write_reg(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF00 => self.io_regs[0] = (val & 0xF0) | (self.io_regs[0] & 0x0F),
            0xFF02 => {
                // Serial transfer control
                self.io_regs[(addr - 0xFF00) as usize] = val;
                if val == 0x81 {
                    print!("{}", self.read_reg(0xFF01)? as char);
                    stdout().flush().unwrap();
                    self.write_reg(0xFF02, 0x01)?;
                }
            }
            0xFF04..=0xFF07 => self.timers.write_reg(addr, val)?,
            _ => self.io_regs[(addr - 0xFF00) as usize] = val,
        }
        Ok(())
    }

    pub(super) fn tick_timers(&mut self) {
        self.timers.tick();
    }

    pub(super) fn check_timer_interrupt(&mut self) -> bool {
        self.timers.check_for_interrupt()
    }

    pub(super) fn change_key(&mut self, button_index: usize, val: bool) {
        self.joypad[button_index] = val;
    }
}
