use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub(super) fn jp_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
        cpu.pc = data.wrapping_sub(3);

        Ok(0)
    }

    pub(super) fn jp_z_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16 if zero flag is set
        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        if cpu.get_zflag() {
            cpu.pc = data.wrapping_sub(3);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jp_nz_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16 if zero flag is not set

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        if !cpu.get_zflag() {
            cpu.pc = data.wrapping_sub(3);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jp_c_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16 if carry flag is set

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        if cpu.get_cflag() {
            cpu.pc = data.wrapping_sub(3);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jp_nc_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address n16 if carry flag is not set

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        if !cpu.get_cflag() {
            cpu.pc = data.wrapping_sub(3);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jr_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address pc + e8
        let data = bus.read_byte(cpu.pc + 1)? as i8; // Immediate signed 8-bit value

        cpu.pc = cpu
            .pc
            .wrapping_add(data as i16 as u16 /* Extend sign to 16-bit*/);

        Ok(0)
    }

    pub(super) fn jr_z_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address pc + e8 if zero flag is set
        let data = bus.read_byte(cpu.pc + 1)? as i8; // Immediate signed 8-bit value 

        if cpu.get_zflag() {
            cpu.pc = cpu
                .pc
                .wrapping_add(data as i16 as u16 /* Extend sign to 16-bit*/);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jr_nz_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address pc + e8 if zero flag is not set
        let data = bus.read_byte(cpu.pc + 1)? as i8; // Immediate signed 8-bit value 

        if !cpu.get_zflag() {
            cpu.pc = cpu
                .pc
                .wrapping_add(data as i16 as u16 /* Extend sign to 16-bit*/);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jr_c_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address pc + e8 if carry flag is set
        let data = bus.read_byte(cpu.pc + 1)? as i8; // Immediate signed 8-bit value 

        if cpu.get_cflag() {
            cpu.pc = cpu
                .pc
                .wrapping_add(data as i16 as u16 /* Extend sign to 16-bit*/);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jr_nc_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Jump to address pc + e8 if carry flag is not set
        let data = bus.read_byte(cpu.pc + 1)? as i8; // Immediate signed 8-bit value 

        if !cpu.get_cflag() {
            cpu.pc = cpu
                .pc
                .wrapping_add(data as i16 as u16 /* Extend sign to 16-bit*/);
            Ok(4)
        } else {
            Ok(0)
        }
    }

    pub(super) fn jp_hl(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy hl to pc

        let data = cpu.hl();

        cpu.pc = data.wrapping_sub(1);

        Ok(0)
    }
}
