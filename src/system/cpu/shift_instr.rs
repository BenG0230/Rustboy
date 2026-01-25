use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
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
}
