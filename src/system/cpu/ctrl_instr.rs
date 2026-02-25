use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub(super) fn nop(_cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        Ok(0)
    }

    pub(super) fn halt(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.halted = true;

        Ok(0)
    }

    pub(super) fn stop(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.stopped = true;

        Ok(0)
    }

    pub(super) fn di(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.ime = false;
        cpu.ime_pending = false;

        Ok(0)
    }

    pub(super) fn ei(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.ime_pending = true;

        Ok(0)
    }
}
