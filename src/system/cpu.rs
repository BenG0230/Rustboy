use crate::system::bus::{Bus, BusError};
use std::fmt;

mod decode;
mod interrupts;

mod arith_instr;
mod bitflag_instr;
mod bitwise_instr;
mod ctrl_instr;
mod jump_instr;
mod ld_instr;
mod misc_instrs;
mod shift_instr;
mod subrout_instr;

pub enum CpuError {
    BusError(BusError),
    RegisterError(u8),
    VecError(u8),
    InstructionError(u8),
    PreInstructionError(u8),
    InterruptError(u8),
}

impl From<BusError> for CpuError {
    fn from(err: BusError) -> CpuError {
        CpuError::BusError(err)
    }
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuError::BusError(err) => write!(f, "{}", err),
            CpuError::RegisterError(err) => write!(f, "Unknown register: {}", err),
            CpuError::VecError(err) => write!(f, "Unknown vector: {}", err),
            CpuError::InstructionError(err) => write!(f, "Illegal instruction: {:#04X}", err),
            CpuError::PreInstructionError(err) => {
                write!(f, "Illegal prefixed instruction: {:#04X}", err)
            }
            CpuError::InterruptError(err) => write!(f, "Unknown interrupt: {:#010b}", err),
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    // --- Registers ---
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16, // Stack pointer
    pc: u16, // Program counter
    // --- Flags ---
    ime: bool,
    // --- Control ---
    halted: bool,
    stopped: bool,
    ime_pending: bool, // Wether to change IME -> True after instruction (see IE)
}

impl Cpu {
    // --- Initialisation
    pub fn new() -> Self {
        //TODO: Change this when implementing a boot rom
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
            ime: false,
            halted: false,
            stopped: false,
            ime_pending: false,
        }
    }

    // --- Flag helpers ---

    pub fn get_zflag(&self) -> bool {
        self.f & 0x80 != 0
    }
    pub fn get_nflag(&self) -> bool {
        self.f & 0x40 != 0
    }
    pub fn get_hflag(&self) -> bool {
        self.f & 0x20 != 0
    }
    pub fn get_cflag(&self) -> bool {
        self.f & 0x10 != 0
    }

    pub fn set_zflag(&mut self, val: bool) {
        if val { self.f |= 0x80 } else { self.f &= 0x7F }
    }
    pub fn set_nflag(&mut self, val: bool) {
        if val { self.f |= 0x40 } else { self.f &= 0xBF }
    }
    pub fn set_hflag(&mut self, val: bool) {
        if val { self.f |= 0x20 } else { self.f &= 0xDF }
    }
    pub fn set_cflag(&mut self, val: bool) {
        if val { self.f |= 0x10 } else { self.f &= 0xEF }
    }

    // --- Register helpers ---

    // Get value in an 8-bit register from index:
    //  0 = b, 1 = c, 2 = d, 3 = e
    //  4 = h, 5 = l, 6 = [HL], 7 = a
    pub fn get_r8(&self, bus: &Bus, reg: u8) -> Result<u8, CpuError> {
        match reg {
            0 => Ok(self.b),
            1 => Ok(self.c),
            2 => Ok(self.d),
            3 => Ok(self.e),
            4 => Ok(self.h),
            5 => Ok(self.l),
            6 => bus.read_byte(self.hl()).map_err(CpuError::from),
            7 => Ok(self.a),
            _ => Err(CpuError::RegisterError(reg)),
        }
    }

    // Write a value to a 8-bit register from index:
    //  0 = b, 1 = c, 2 = d, 3 = e
    //  4 = h, 5 = l, 6 = [HL], 7 = a
    pub fn set_r8(&mut self, bus: &mut Bus, reg: u8, val: u8) -> Result<(), CpuError> {
        match reg {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            6 => bus.write_byte(self.hl(), val).map_err(CpuError::from)?,
            7 => self.a = val,
            _ => Err(CpuError::RegisterError(reg))?,
        }
        Ok(())
    }

    // Get value in a 16-bit register from index:
    // 0 = bc, 1 = de
    // 2 = hl, 3 = sp
    pub fn get_r16(&self, reg: u8) -> Result<u16, CpuError> {
        match reg {
            0 => Ok(self.bc()),
            1 => Ok(self.de()),
            2 => Ok(self.hl()),
            3 => Ok(self.sp),
            _ => Err(CpuError::RegisterError(reg)),
        }
    }

    // Get value in a 16-bit register from index:
    // 0 = bc, 1 = de
    // 2 = hl, 3 = sp
    pub fn set_r16(&mut self, reg: u8, val: u16) -> Result<(), CpuError> {
        match reg {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.sp = val,
            _ => Err(CpuError::RegisterError(reg))?,
        }
        Ok(())
    }

    // Get addr stored in 16-bit register from index:
    // 0 = bc, 1 = de
    // 2 = hl+, 3 = hl-
    pub fn get_r16_mem(&mut self, reg: u8) -> Result<u16, CpuError> {
        match reg {
            0 => Ok(self.bc()),
            1 => Ok(self.de()),
            2 => {
                let temp = self.hl();
                self.set_hl(self.hl().wrapping_add(1));
                Ok(temp)
            }
            3 => {
                let temp = self.hl();
                self.set_hl(self.hl().wrapping_sub(1));
                Ok(temp)
            }
            _ => Err(CpuError::RegisterError(reg)),
        }
    }

    // Get value stored in 16-bit register from index for stack operations:
    // 0 = bc, 1 = de,
    // 2 = hl, 3 = af
    pub fn get_r16_stk(&self, reg: u8) -> Result<u16, CpuError> {
        match reg {
            0 => Ok(self.bc()),
            1 => Ok(self.de()),
            2 => Ok(self.hl()),
            3 => Ok(self.af()),
            _ => Err(CpuError::RegisterError(reg)),
        }
    }

    // Set value stored in 16-bit register from index for stack operations:
    // 0 = bc, 1 = de,
    // 2 = hl, 3 = af
    pub fn set_r16_stk(&mut self, reg: u8, val: u16) -> Result<(), CpuError> {
        match reg {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.set_af(val),
            _ => Err(CpuError::RegisterError(reg))?,
        }
        Ok(())
    }

    // --- 16-bit register helpers ---

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    // --- Misc helpers ---

    pub fn get_vec(&self, vec: u8) -> Result<u16, CpuError> {
        match vec {
            0 => Ok(0x0000),
            1 => Ok(0x0008),
            2 => Ok(0x0010),
            3 => Ok(0x0018),
            4 => Ok(0x0020),
            5 => Ok(0x0028),
            6 => Ok(0x0030),
            7 => Ok(0x0038),
            _ => Err(CpuError::VecError(vec)),
        }
    }

    // --- Emulation ---
    pub fn step(&mut self, bus: &mut Bus) -> Result<u8, CpuError> {
        // TODO: Interrupts

        let ie = bus.read_byte(0xFFFF)?;
        let if_reg = bus.read_byte(0xFF0F)?;
        if self.ime && (ie & if_reg) != 0 {
            self.service_interrupts(bus)?;
            return Ok(20); // servicing interrupt takes 5 m-cycles
        }

        if !(self.stopped) {
            if !(self.halted) {
                return self.decode(bus);
            }
        }

        Ok(0)
    }
}
