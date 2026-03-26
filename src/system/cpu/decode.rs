use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

mod instruct_table;
mod prefix_table;

static INSTRUCTION_TABLE: [Instruction; 256] = instruct_table::load_instruction_table();
static PREFIX_TABLE: [Instruction; 256] = prefix_table::load_prefix_table();

#[derive(Copy, Clone)]
pub struct Instruction {
    cycles: u8,
    bytes: u16,
    helper: fn(&mut Cpu, &mut Bus, u8) -> Result<u8, CpuError>,
    mneumonic: &'static str,
}

impl Instruction {
    pub const fn new(
        cycles: u8,
        bytes: u16,
        helper: fn(&mut Cpu, &mut Bus, u8) -> Result<u8, CpuError>,
        mneumonic: &'static str,
    ) -> Self {
        Self {
            cycles,
            bytes,
            helper,
            mneumonic,
        }
    }

    pub fn execute(&self, cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        (self.helper)(cpu, bus, opcode)
    }
}

impl Cpu {
    fn fetch(&self, bus: &mut Bus) -> Result<u8, CpuError> {
        bus.read_byte(self.pc).map_err(CpuError::from)
    }

    pub fn decode(&mut self, bus: &mut Bus) -> Result<u8, CpuError> {
        let ime_pending = self.ime_pending;

        let mut opcode = self.fetch(bus)?;

        let instruction = if opcode == 0xCB {
            // Take from prefix table instead
            self.pc = self.pc.wrapping_add(1);
            opcode = self.fetch(bus)?;
            PREFIX_TABLE[opcode as usize]
        } else {
            INSTRUCTION_TABLE[opcode as usize]
        };

        self.trace_print(bus, instruction);

        let extra_cycles = instruction.execute(self, bus, opcode)?;
        self.pc = self.pc.wrapping_add(instruction.bytes);

        if ime_pending {
            self.ime = true;
            self.ime_pending = false;
        }

        Ok(instruction.cycles + extra_cycles)
    }

    fn trace_print(&self, bus: &mut Bus, instruction: Instruction) {
        println!(
            "AF:{:02X}{:02X} BC:{:02X}{:02X} DE:{:02X}{:02X} HL:{:02X}{:02X} SP:{:04X} PC:{:04X} -> {} SCX:{:02X} SCY{:02X}",
            self.a,
            self.f,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            instruction.mneumonic,
            bus.read_byte(0xFF43).unwrap(),
            bus.read_byte(0xFF42).unwrap(),
        );
    }
}
