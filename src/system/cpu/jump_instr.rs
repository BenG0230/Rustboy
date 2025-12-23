use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub fn jp_n16(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
        cpu.pc = data - 3;

        Ok(0)
    }

    pub fn jp_nz_n16(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16 if zero flag is not set

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        if cpu.get_zflag() {
            Ok(0)
        } else {
            cpu.pc = data - 3;
            Ok(4)
        }
    }
}
