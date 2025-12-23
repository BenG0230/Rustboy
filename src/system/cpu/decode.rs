use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

mod instruct_table;

static INSTRUCTION_TABLE: [Instruction; 256] = instruct_table::load_instruction_table();

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
    pub fn unknown_instr(_cpu: &mut Cpu, _bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Either Instruction not implemented or tis illegal
        Err(CpuError::InstructionError(opcode))
    }

    fn fetch(&self, bus: &mut Bus) -> Result<u8, CpuError> {
        bus.read_byte(self.pc).map_err(CpuError::from)
    }

    pub fn decode(&mut self, bus: &mut Bus) -> Result<u8, CpuError> {
        let ime_pending = self.ime_pending;

        let opcode = self.fetch(bus)?;

        let instruction = INSTRUCTION_TABLE[opcode as usize];

        // println!(
        //     "{:#06X}: {:02X} -> {}",
        //     self.pc, opcode, instruction.mneumonic
        // );

        let extra_cycles = instruction.execute(self, bus, opcode)?;
        self.pc = self.pc.wrapping_add(instruction.bytes);

        if ime_pending {
            self.ime = true;
            self.ime_pending = false;
        }

        Ok(instruction.cycles + extra_cycles)
    }
}
