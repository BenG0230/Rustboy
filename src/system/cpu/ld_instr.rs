use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub fn ld_r8_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Copy value from one r8 to another
        let source = opcode & 0b00000111; // Source register "index"
        let dest = (opcode & 0b00111000) >> 3; // Destination register "index"

        // Extra cycles for instructions including [HL]
        let cycles = if source == 6 || dest == 6 { 4 } else { 0 };

        // Get and set data
        let data = cpu.get_r8(bus, source)?;
        cpu.set_r8(bus, dest, data)?;

        Ok(cycles)
    }

    pub fn ld_r8_n8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Copy n8 into r8
        let data = bus.read_byte(cpu.pc + 1)?; // Data from Immediate val
        let dest = (opcode & 0b00111000) >> 3; // Destination register "index"

        // Extra cycles for instruction including [HL]
        let cycles = if dest == 6 { 4 } else { 0 };

        // Set data
        cpu.set_r8(bus, dest, data)?;

        Ok(cycles)
    }

    pub fn ld_r16_n16(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Copy n16 into r16
        // data little-endian 16-bit immediate val
        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
        let dest = (opcode & 0b00110000) >> 4;

        // Set data
        cpu.set_r16(dest, data)?;

        Ok(0)
    }

    pub fn ld_r16mem_a(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Copy value from A to [r16]
        let data = cpu.a; // Data from A register 
        let dest = (opcode & 0b00110000) >> 4; // r16 containing destination
        let addr = cpu.get_r16_mem(dest)?; // destination address

        // Set data
        bus.write_byte(addr, data)?;

        Ok(0)
    }

    pub fn ld_n16mem_a(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy value from A to [n16]
        let data = cpu.a; // Data from A register
        // addr from little-endian 16-bit immediate val
        let addr = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        // Set data
        bus.write_byte(addr, data)?;

        Ok(0)
    }

    pub fn ldh_n8mem_a(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy value from A to [0xFF00 + n8]
        let data = cpu.a; // Data from A register
        let addr = 0xFF00 + bus.read_byte(cpu.pc + 1)? as u16; // Address from Immediate val

        bus.write_byte(addr, data)?;

        Ok(0)
    }

    pub fn ldh_cmem_a(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy value from A to [0xFF00 + C]
        let data = cpu.a; // Data from A register
        let addr = 0xFF00 + cpu.c as u16; // Address from 0xFF00 + C

        // Set data
        bus.write_byte(addr, data)?;

        Ok(0)
    }

    pub fn ld_a_r16mem(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Copy data from [r16] to A
        let source = (opcode & 0b00110000) >> 4; // r16 containing source
        let addr = cpu.get_r16_mem(source)?; // Address of data
        let data = bus.read_byte(addr)?;

        // Set data
        cpu.a = data;

        Ok(0)
    }

    pub fn ld_a_n16mem(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy data from [n16] to A
        // Address of data
        let addr = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
        let data = bus.read_byte(addr)?;

        // Set data
        cpu.a = data;

        Ok(0)
    }

    pub fn ld_a_n8mem(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy data from [0xFF00 + n8] to A
        let addr = 0xFF00 + bus.read_byte(cpu.pc + 1)? as u16;
        let data = bus.read_byte(addr)?;

        cpu.a = data;

        Ok(0)
    }

    pub fn ld_a_cmem(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy data from [0xFF00 + C] to A
        let addr = 0xFF00 + cpu.c as u16;
        let data = bus.read_byte(addr)?;

        cpu.a = data;

        Ok(0)
    }
}
