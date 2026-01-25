use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    // --- 8-bit ---
    pub fn add_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Add r8 to A
        let source = opcode & 0b00000111; // Source register index from opcode
        let data = cpu.get_r8(bus, source)?; // Data from register
        let a = cpu.a;

        let result = a.wrapping_add(data); // Calculate value

        // Add 4 cycles if includes [HL]
        let cycles = if source == 6 { 4 } else { 0 };

        // Set data
        cpu.a = result;

        // Set flags
        cpu.set_zflag(result == 0); // If result equals 0
        cpu.set_nflag(false); // always 0

        cpu.set_hflag(((data & 0xF) + (a & 0xF)) > 0xF); // If lower nibble overflow
        cpu.set_hflag(((data as u16) + (a as u16)) > 0xFF); // If overflow

        Ok(cycles)
    }

    pub fn add_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Add n8 to A
        let data = bus.read_byte(cpu.pc + 1)?; // Immediate 8-bit value
        let a = cpu.a;

        let result = a.wrapping_add(data); // Calculate value

        // Set data
        cpu.a = result;

        // Set flags
        cpu.set_zflag(result == 0); // If result equals 0
        cpu.set_nflag(false); // always 0

        cpu.set_hflag(((data & 0xF) + (a & 0xF)) > 0xF); // If lower nibble overflow
        cpu.set_hflag(((data as u16) + (a as u16)) > 0xFF); // If overflow

        Ok(0)
    }

    pub fn adc_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Add r8 + carry to A

        let source = opcode & 0b00000111; // Source register index from opcode
        let data = cpu.get_r8(bus, source)?; // Data from register
        let a = cpu.a;
        let carry = if cpu.get_cflag() { 1 } else { 0 };

        let result = ((data as u16) + (a as u16) + (carry as u16)) as u8; // Calculate value

        // Add 4 cycles if includes [HL]
        let cycles = if source == 6 { 4 } else { 0 };

        // Set data
        cpu.a = result;

        // Set flags
        cpu.set_zflag(result == 0); // If zero
        cpu.set_nflag(false); // always 0

        cpu.set_hflag(((data & 0xF) + (a & 0xF) + carry) > 0xF); // If overflow from low nibble
        cpu.set_cflag(((data as u16) + (a as u16) + (carry as u16)) > 0xFF); // If overflow

        Ok(cycles)
    }

    pub fn adc_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Add n8 + carry to A

        let data = bus.read_byte(cpu.pc + 1)?; // Immediate 8-bit value
        let a = cpu.a;
        let carry = if cpu.get_cflag() { 1 } else { 0 };

        let result = ((data as u16) + (a as u16) + (carry as u16)) as u8; // Calculate value

        // Set data
        cpu.a = result;

        // Set flags
        cpu.set_zflag(result == 0); // If zero
        cpu.set_nflag(false); // always 0

        cpu.set_hflag(((data & 0xF) + (a & 0xF) + carry) > 0xF); // If overflow from low nibble
        cpu.set_cflag(((data as u16) + (a as u16) + (carry as u16)) > 0xFF); // If overflow

        Ok(0)
    }

    pub fn sub_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Subtract r8 from a

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let a = cpu.a;

        let result = a.wrapping_sub(data);

        let cycles = if source == 6 { 4 } else { 0 };

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(true);

        cpu.set_hflag((data & 0xF) > (a & 0xF)); // If borrow from bit 4
        cpu.set_cflag(data > a); // If borrow

        Ok(cycles)
    }

    pub fn sub_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Subtract n8 from a
        let data = bus.read_byte(cpu.pc + 1)?;
        let a = cpu.a;

        let result = a.wrapping_sub(data);

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(true);

        cpu.set_hflag((data & 0xF) > (a & 0xF)); // If borrow from bit 4
        cpu.set_cflag(data > a); // If borrow

        Ok(0)
    }

    pub fn sbc_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Subtract r8 and carry from a

        let source = opcode & 0b00000111;
        let data = cpu.get_r8(bus, source)?;
        let a = cpu.a;
        let carry = cpu.get_cflag();

        let result = ((a as u16) - (data as u16) - (carry as u16)) as u8;

        let cycles = if source == 6 { 4 } else { 0 };

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(true);

        cpu.set_hflag((data & 0xF) > (a & 0xF)); // If borrow from bit 4
        cpu.set_cflag(data > a); // If borrow

        Ok(cycles)
    }

    pub fn sbc_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Subtract r8 and carry from a

        let data = bus.read_byte(cpu.pc + 1)?; // Immediate 8-bit value
        let a = cpu.a;
        let carry = cpu.get_cflag();

        let result = ((a as u16) - (data as u16) - (carry as u16)) as u8;

        cpu.a = result;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(true);

        cpu.set_hflag((data & 0xF) > (a & 0xF)); // If borrow from bit 4
        cpu.set_cflag(data > a); // If borrow

        Ok(0)
    }

    pub fn cp_a_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Compare A with r8

        let source = opcode & 0b00000111; // Source register index from opcode
        let data = cpu.get_r8(bus, source)?; // Data from register
        let a = cpu.a;

        let result = a.wrapping_sub(data); // Calculate value

        // Add 4 cycles if includes [HL]
        let cycles = if source == 6 { 4 } else { 0 };

        // set flags
        cpu.set_zflag(result == 0); // If zero
        cpu.set_nflag(true); // always 1

        cpu.set_hflag((data & 0xF) > (a & 0xF)); // If borrow from bit 4
        cpu.set_cflag(data > a); // If borrow

        Ok(cycles)
    }

    pub fn cp_a_n8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Compare A with n8

        let data = bus.read_byte(cpu.pc + 1)?;
        let a = cpu.a;

        let result = a.wrapping_sub(data); // Calculate value

        // set flags
        cpu.set_zflag(result == 0); // If zero
        cpu.set_nflag(true); // always 1

        cpu.set_hflag((data & 0xF) > (a & 0xF)); // If borrow from bit 4
        cpu.set_cflag(data > a); // If borrow

        Ok(0)
    }

    pub fn inc_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Increment r8

        let source = (opcode & 0b00111000) >> 3;
        let data = cpu.get_r8(bus, source)?;
        let result = data.wrapping_add(1);

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(false);

        cpu.set_hflag((data & 0xF) == 0xF);

        Ok(cycles)
    }

    pub fn dec_r8(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Decrement r8

        let source = (opcode & 0b00111000) >> 3;
        let data = cpu.get_r8(bus, source)?;
        let result = data.wrapping_sub(1);

        let cycles = if source == 6 { 8 } else { 0 };

        cpu.set_r8(bus, source, result)?;

        cpu.set_zflag(result == 0);
        cpu.set_nflag(true);

        cpu.set_hflag(data == 0x10);

        Ok(cycles)
    }

    // --- 16-bit ---

    pub fn inc_r16(cpu: &mut Cpu, _bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Increment r16 by 1

        let source = (opcode & 0b00110000) >> 4;
        let data = cpu.get_r16(source)?;

        cpu.set_r16(source, data.wrapping_add(1))?;

        Ok(0)
    }

    pub fn dec_r16(cpu: &mut Cpu, _bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Decrement r16 by 1

        let source = (opcode & 0b00110000) >> 4;
        let data = cpu.get_r16(source)?;

        cpu.set_r16(source, data.wrapping_sub(1))?;

        Ok(0)
    }

    pub fn add_hl_r16(cpu: &mut Cpu, _bus: &mut Bus, opcode: u8) -> Result<u8, CpuError> {
        // Add r16 to HL

        let source = (opcode & 0b00110000) >> 4;
        let data = cpu.get_r16(source)?;
        let hl = cpu.hl();

        let result = hl.wrapping_add(data);

        cpu.set_hl(result);

        cpu.set_nflag(false);

        cpu.set_hflag(((data & 0xFFF) + (hl & 0xFFF)) > 0xFFF);
        cpu.set_cflag(((data as u32) + (hl as u32)) > 0xFFFF);

        Ok(0)
    }

    pub fn add_sp_e8(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Add e8 to SP

        let data = bus.read_byte(cpu.pc + 1)? as i8 as i16 as u16; // extend sign to 16-bit
        let sp = cpu.sp;

        let result = sp.wrapping_add(data);
        cpu.sp = result;

        cpu.set_zflag(false);
        cpu.set_nflag(false);

        cpu.set_hflag((sp & 0x0F) + (data & 0x0F) > 0x0F);
        cpu.set_cflag((sp & 0xFF) + (data & 0xFF) > 0xFF);

        Ok(0)
    }
}
