use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub(super) fn unknown_instr(
        _cpu: &mut Cpu,
        _bus: &mut Bus,
        opcode: u8,
    ) -> Result<u8, CpuError> {
        // Either Instruction not implemented or tis illegal
        Err(CpuError::InstructionError(opcode))
    }

    pub(super) fn unknown_pre_instr(
        _cpu: &mut Cpu,
        _bus: &mut Bus,
        opcode: u8,
    ) -> Result<u8, CpuError> {
        Err(CpuError::PreInstructionError(opcode))
    }

    pub(super) fn daa(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Change A into BCD

        let mut adjustment = 0;
        let a = cpu.a;
        let result;

        if cpu.get_nflag() {
            // Subtract flag is set

            if cpu.get_hflag() {
                adjustment += 0x6;
            }

            if cpu.get_cflag() {
                adjustment += 0x60
            }

            result = a.wrapping_sub(adjustment);
        } else {
            if cpu.get_hflag() || (a & 0xF) > 0x9 {
                adjustment += 0x6;
            }

            if cpu.get_cflag() || a > 0x99 {
                adjustment += 0x60;
                cpu.set_cflag(true);
            } else {
                cpu.set_cflag(false);
            }

            result = a.wrapping_add(adjustment);
        }

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_hflag(false);

        Ok(0)
    }

    pub(super) fn scf(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Set the carry flag

        cpu.set_nflag(false);
        cpu.set_hflag(false);

        cpu.set_cflag(true);

        Ok(0)
    }

    pub(super) fn ccf(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Invert carry flag

        cpu.set_nflag(false);
        cpu.set_hflag(false);

        cpu.set_cflag(!cpu.get_cflag());

        Ok(0)
    }
}
