use crate::system::bus::BusError;

pub struct Io {
    io_regs: [u8; 128],
}

impl Io {
    pub fn new() -> Self {
        Self { io_regs: [0; 128] }
    }

    pub fn read_reg(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            _ => Ok(self.io_regs[(addr - 0xFF00) as usize]),
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF02 => {
                // Serial transfer control
                self.io_regs[(addr - 0xFF00) as usize] = val;
                if val == 0x81 {
                    print!("{}", self.read_reg(0xFF01)? as char);
                    self.write_reg(0xFF02, 0x01)?;
                }
            }
            _ => self.io_regs[(addr - 0xFF00) as usize] = val,
        }
        Ok(())
    }
}
