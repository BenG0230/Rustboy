use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

mod instruct_table;

#[derive(Copy, Clone)]
pub struct Instruction {
    cycles: u8,
    bytes: u8,
    helper: fn(&mut Cpu, &mut Bus, u8) -> Result<u8, CpuError>,
    mneumonic: &'static str,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            cycles: 0,
            bytes: 0,
            helper: Cpu::unknown_instr,
            mneumonic: "Unkown :P",
        }
    }
}

impl Instruction {
    pub fn new(
        cycles: u8,
        bytes: u8,
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

    fn fetch(&self, bus: &mut Bus) -> u8 {
        match bus.read_byte(self.pc) {
            Ok(opcode) => opcode,
            Err(e) => {
                println!("Error: {}", e);
                0x10 // Return stop if error occured (i guess :P)
                // TODO: Change this if it is bad
            }
        }
    }

    fn decode(&self, opcode: u8) {
        println!("Im decoding {:#02X}", opcode);
    }
}
