use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    // Non-prefixed

    pub fn rla(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Rotate A left through carry flag

        let a = cpu.a;
        let c = cpu.get_cflag();

        cpu.a = a << 1 | c as u8;

        cpu.set_zflag(false);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((a & 0b10000000) > 0);

        Ok(0)
    }

    pub fn rlca(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Rotate A left

        let a = cpu.a;

        cpu.a = a << 1 | ((a & 0b10000000) >> 7);

        cpu.set_zflag(false);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((a & 0b10000000) > 0);

        Ok(0)
    }

    pub fn rra(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Rotate A right through carry flag

        let a = cpu.a;
        let c = cpu.get_cflag();

        cpu.a = a >> 1 | ((c as u8) << 7);

        cpu.set_zflag(false);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((a & 1) > 0);

        Ok(0)
    }

    pub fn rrca(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Rotate A right

        let a = cpu.a;

        cpu.a = a >> 1 | ((a & 1) << 7);

        cpu.set_zflag(false);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((a & 1) > 0);

        Ok(0)
    }

    // Prefixed
    pub fn rlc_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Rotate r8 left

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;

        let result = data << 1 | ((data & 0b10000000) >> 7);

        cpu.set_r8(bus, source, result)?;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(false);
        cpu.set_cflag((data & 0b10000000) > 0);

        Ok(cycles)
    }

    pub fn rrc_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Rotate r8 right

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;

        let result = data >> 1 | ((data & 1) << 7);

        cpu.set_r8(bus, source, result)?;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);
        cpu.set_hflag(false);
        cpu.set_cflag((data & 1) > 0);

        Ok(cycles)
    }

    pub fn rl_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Rotate r8 left through carry flag

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let c = cpu.get_cflag();

        let result = data << 1 | c as u8;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((data & 0b10000000) > 0);

        Ok(cycles)
    }

    pub fn rr_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Rotate r8 right through carry flag

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let c = cpu.get_cflag();

        let result = data >> 1 | ((c as u8) << 7);

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((data & 1) > 0);

        Ok(cycles)
    }

    pub fn sla_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Arithmetically left shift r8

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;

        let result = data << 1;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((data & 0b10000000) > 0);

        Ok(cycles)
    }

    pub fn sra_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Arithmetically right shift r8

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;

        let result = data >> 1 | (data & 0b10000000);

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((data & 1) > 0);

        Ok(cycles)
    }

    pub fn swap_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Swap the high nibble and low nibble

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;

        let result = (data & 0x0F) << 4 | (data & 0xF0) >> 4;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag(false);

        Ok(cycles)
    }

    pub fn srl_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Logically right shift r8

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;

        let result = data >> 1;

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag(false);
        cpu.set_cflag((data & 1) > 0);

        Ok(cycles)
    }
}
