mod timer;

use crate::system::bus::BusError;
use std::io::Write;
use std::io::stdout;
use timer::Timers;

pub struct Io {
    io_regs: [u8; 128],
    timers: Timers,
}

impl Io {
    pub fn new() -> Self {
        Self {
            io_regs: [0xFF; 128],
            timers: Timers::new(),
        }
    }

    pub(super) fn read_reg(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF04..=0xFF07 => self.timers.read_reg(addr),
            _ => Ok(self.io_regs[(addr - 0xFF00) as usize]),
        }
    }

    pub(super) fn write_reg(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF00 => {
                self.io_regs[0] = (val & 0xF0) | (self.io_regs[0] & 0x0F);
                Ok(())
            }
            0xFF02 => {
                // Serial transfer control
                self.io_regs[(addr - 0xFF00) as usize] = val;
                if val == 0x81 {
                    print!("{}", self.read_reg(0xFF01)? as char);
                    stdout().flush().unwrap();
                    self.write_reg(0xFF02, 0x01)?;
                }

                Ok(())
            }
            0xFF04..=0xFF07 => self.timers.write_reg(addr, val),
            _ => {
                self.io_regs[(addr - 0xFF00) as usize] = val;
                Ok(())
            }
        }
    }

    pub(super) fn tick_timers(&mut self) {
        self.timers.tick();
    }

    pub(super) fn check_timer_interrupt(&mut self) -> bool {
        self.timers.check_for_interrupt()
    }
}
