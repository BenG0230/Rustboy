use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub(super) fn bit_b3_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Set zero flag if bit b3 of r8 is set

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let bit_index = (opcode & 0b00111000) >> 3;
        let bit_mask = 1 << bit_index;

        let cycles = if source == 6 { 4 } else { 0 };

        cpu.set_zflag((data & bit_mask) == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(true);

        Ok(cycles)
    }

    pub(super) fn res_b3_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Set bit b3 of r8 to 0

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let bit_index = (opcode & 0b00111000) >> 3;
        let bit_mask = !(1 << bit_index);

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, data & bit_mask)?;

        Ok(cycles)
    }

    pub(super) fn set_b3_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Set bit b3 of r8 to 1
        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let bit_index = (opcode & 0b00111000) >> 3;
        let bit_mask = 1 << bit_index;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, data | bit_mask)?;

        Ok(cycles)
    }
}
