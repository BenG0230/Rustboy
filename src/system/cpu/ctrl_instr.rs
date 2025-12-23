use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub fn nop(_cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        Ok(0)
    }

    pub fn halt(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.halted = true;

        Ok(0)
    }

    pub fn stop(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.stopped = true;

        Ok(0)
    }

    pub fn di(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.ime = false;

        Ok(0)
    }

    pub fn ie(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        cpu.ime_pending = true;

        Ok(0)
    }
}
