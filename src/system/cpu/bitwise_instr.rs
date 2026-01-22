use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub fn and_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Bitwise AND A and r8

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let a = cpu.a;

        let result = a & data;

        let cycles = if source == 6 { 4 } else { 0 };

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(true);
        cpu.set_cflag(false);

        Ok(cycles)
    }

    pub fn and_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Bitwise AND A and r8

        let data = bus.read_byte(cpu.pc + 1)?;
        let a = cpu.a;

        let result = a & data;

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(true);
        cpu.set_cflag(false);

        Ok(0)
    }
}
