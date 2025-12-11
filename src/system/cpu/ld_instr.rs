use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub fn unknown_instr(_cpu: &mut Cpu, _bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        Err(CpuError::InstructionError(opcode))
    }

    pub fn ld_r8_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        let source = opcode & 0b00000111; // Source register "index"
        let dest = (opcode & 0b00111000) >> 3; // Destination register "index"

        // Extra cycles for instructions including [HL]
        let cycles = if source == 6 || dest == 6 { 4 } else { 0 };

        // Get and set data
        let data = cpu.get_r8(bus, source)?;
        cpu.set_r8(bus, dest, data)?;

        Ok(cycles)
    }

    pub fn ld_r8_n8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        let data = bus.read_byte(cpu.pc + 1).map_err(CpuError::from)?; // Data from Immediate val
        let dest = (opcode & 0b00111000) >> 3; // Destination register "index"

        // Extra cycles for instruction including [HL]
        let cycles = if dest == 6 { 4 } else { 0 };

        // Set data
        cpu.set_r8(bus, dest, data)?;

        Ok(cycles)
    }

    pub fn ld_r16_n16(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // data little-endian 16-bit immediate val
        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
        let dest = (opcode & 0b00110000) >> 4;

        cpu.set_r16(dest, data)?;

        Ok(0)
    }
}
