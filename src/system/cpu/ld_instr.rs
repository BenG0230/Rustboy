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

        // Set data
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
        let addr = 0xFF00 + bus.read_byte(cpu.pc + 1)? as u16; // Get address from immediate value
        let data = bus.read_byte(addr)?; // Get data from address

        // Set data
        cpu.a = data;

        Ok(0)
    }

    pub fn ld_a_cmem(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy data from [0xFF00 + C] to A
        let addr = 0xFF00 + cpu.c as u16; // Get address from C
        let data = bus.read_byte(addr)?; // Get data from address

        // Set data
        cpu.a = data;

        Ok(0)
    }

    pub fn ld_n16mem_sp(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy SP into memory starting at [n16]
        // Split SP into high and low Bytes
        let data_high = (cpu.sp >> 8) as u8;
        let data_low = (cpu.sp & 0xFF) as u8;
        // Get address from immediate value
        let addr = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;

        // Write both Bytes
        bus.write_byte(addr, data_low)?;
        bus.write_byte(addr + 1, data_high)?;

        Ok(0)
    }

    pub fn ld_hl_sp_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Load SP + e8 to HL

        let data_sp = cpu.sp; // Data from sp
        let data_imm = bus.read_byte(cpu.pc + 1)? as i8; // Data from immediate signed n8

        cpu.set_hl(data_sp.wrapping_add(data_imm as i16 as u16 /* extend sign to 16-bit*/));

        // z + n flags -> 0
        cpu.set_zflag(false);
        cpu.set_nflag(false);

        // h -> 1 if overflow from lower nibble else -> 0
        // c -> 1 if overflow from lower byte else -> 0
        cpu.set_hflag((data_sp & 0x0F) + ((data_imm as i16 as u16) & 0x0F) > 0x0F);
        cpu.set_cflag((data_sp & 0xFF) + ((data_imm as i16 as u16) & 0xFF) > 0xFF);

        Ok(0)
    }

    pub fn ld_sp_hl(cpu: &mut Cpu, _bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Copy HL to SP
        let data = cpu.hl();
        cpu.sp = data;

        Ok(0)
    }

    pub fn pop_r16stk(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Pop value from stack to r16stk
        let dest = (opcode & 0b00110000) >> 4; // Get 

        let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
        cpu.sp += 1; // increment stack pointer
        let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
        cpu.sp += 1; // Increment stack pointer

        // get full 16-bit data
        let data = data_high | data_low;

        // Set data
        cpu.set_r16_stk(dest, data)?;

        Ok(0)
    }

    pub fn push_r16stk(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Push value from r16stk to stack

        let source = (opcode & 0b00110000) >> 4;

        let data = cpu.get_r16_stk(source)?; // Get data from register
        // Split data into low and high bit
        let data_low = data as u8;
        let data_high = ((data & 0xFF00) >> 8) as u8;

        // Put data in memory
        cpu.sp -= 1;
        bus.write_byte(cpu.sp, data_high)?;
        cpu.sp -= 1;
        bus.write_byte(cpu.sp, data_low)?;

        Ok(0)
    }
}
