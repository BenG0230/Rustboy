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
        // Bitwise AND A and n8

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

    pub fn xor_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // bitwise xor a and r8

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let a = cpu.a;

        let result = a ^ data;

        let cycles = if source == 6 { 4 } else { 0 };

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(false);
        cpu.set_cflag(false);

        Ok(cycles)
    }

    pub fn xor_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // bitwise xor a and n8

        let data = bus.read_byte(cpu.pc + 1)?;
        let a = cpu.a;

        let result = a ^ data;

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(false);
        cpu.set_cflag(false);

        Ok(0)
    }

    pub fn or_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Bitwise OR A and r8

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let a = cpu.a;

        let result = a | data;

        let cycles = if source == 6 { 4 } else { 0 };

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(false);
        cpu.set_cflag(false);

        Ok(cycles)
    }

    pub fn or_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Bitwise OR A and r8

        let data = bus.read_byte(cpu.pc + 1)?;
        let a = cpu.a;

        let result = a | data;

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(false);
        cpu.set_cflag(false);

        Ok(0)
    }

    pub fn cpl(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Bitwise NOT

        cpu.a = !cpu.a;

        cpu.set_nflag(true);
        cpu.set_hflag(true);

        Ok(0)
    }
}
