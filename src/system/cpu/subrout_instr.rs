use crate::system::{
    bus::Bus,
    cpu::{Cpu, CpuError},
};

impl Cpu {
    pub fn call_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Push the next instruction address onto stack and jump to address n16

        let data = ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
        let next_addr = cpu.pc.wrapping_add(3);
        let next_addr_low = next_addr as u8;
        let next_addr_high = ((next_addr & 0xFF00) >> 8) as u8;

        cpu.sp -= 1;
        bus.write_byte(cpu.sp, next_addr_high)?;
        cpu.sp -= 1;
        bus.write_byte(cpu.sp, next_addr_low)?;

        cpu.pc = data.wrapping_sub(3);

        Ok(0)
    }

    pub fn call_z_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If z flag is set
        // Push the next instruction onto stack and jump to address n16

        if cpu.get_zflag() {
            let data =
                ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
            let next_addr = cpu.pc.wrapping_add(3);
            let next_addr_low = next_addr as u8;
            let next_addr_high = ((next_addr & 0xFF00) >> 8) as u8;

            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_high)?;
            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_low)?;

            cpu.pc = data.wrapping_sub(3);

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn call_nz_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If z flag is unset
        // Push the next instruction onto stack and jump to address n16

        if !cpu.get_zflag() {
            let data =
                ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
            let next_addr = cpu.pc.wrapping_add(3);
            let next_addr_low = next_addr as u8;
            let next_addr_high = ((next_addr & 0xFF00) >> 8) as u8;

            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_high)?;
            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_low)?;

            cpu.pc = data.wrapping_sub(3);

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn call_c_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If c flag is set
        // Push the next instruction onto stack and jump to address n16

        if cpu.get_cflag() {
            let data =
                ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
            let next_addr = cpu.pc.wrapping_add(3);
            let next_addr_low = next_addr as u8;
            let next_addr_high = ((next_addr & 0xFF00) >> 8) as u8;

            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_high)?;
            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_low)?;

            cpu.pc = data.wrapping_sub(3);

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn call_nc_n16(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If c flag is unset
        // Push the next instruction onto stack and jump to address n16

        if !cpu.get_cflag() {
            let data =
                ((bus.read_byte(cpu.pc + 2)? as u16) << 8) | bus.read_byte(cpu.pc + 1)? as u16;
            let next_addr = cpu.pc.wrapping_add(3);
            let next_addr_low = next_addr as u8;
            let next_addr_high = ((next_addr & 0xFF00) >> 8) as u8;

            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_high)?;
            cpu.sp -= 1;
            bus.write_byte(cpu.sp, next_addr_low)?;

            cpu.pc = data.wrapping_sub(3);

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn ret(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Pop the value on the stack to PC

        let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
        cpu.sp += 1; // increment stack pointer
        let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
        cpu.sp += 1; // Increment stack pointer

        // get full 16-bit data
        let data = data_high | data_low;

        cpu.pc = data;

        Ok(0)
    }

    pub fn ret_z(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If z flag is set
        // Pop the value on the stack to PC

        if cpu.get_zflag() {
            let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
            cpu.sp += 1; // increment stack pointer
            let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
            cpu.sp += 1; // Increment stack pointer

            // get full 16-bit data
            let data = data_high | data_low;

            cpu.pc = data;

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn ret_nz(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If z flag is unset
        // Pop the value on the stack to PC

        if !cpu.get_zflag() {
            let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
            cpu.sp += 1; // increment stack pointer
            let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
            cpu.sp += 1; // Increment stack pointer

            // get full 16-bit data
            let data = data_high | data_low;

            cpu.pc = data;

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn ret_c(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If c flag is set
        // Pop the value on the stack to PC

        if cpu.get_cflag() {
            let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
            cpu.sp += 1; // increment stack pointer
            let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
            cpu.sp += 1; // Increment stack pointer

            // get full 16-bit data
            let data = data_high | data_low;

            cpu.pc = data;

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn ret_nc(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // If c flag is unset
        // Pop the value on the stack to PC

        if !cpu.get_cflag() {
            let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
            cpu.sp += 1; // increment stack pointer
            let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
            cpu.sp += 1; // Increment stack pointer

            // get full 16-bit data
            let data = data_high | data_low;

            cpu.pc = data;

            Ok(12)
        } else {
            Ok(0)
        }
    }

    pub fn reti(cpu: &mut Cpu, bus: &mut Bus, _opcode: u8) -> Result<u8, CpuError> {
        // Pop the value on the stack to PC then set IME

        let data_low = bus.read_byte(cpu.sp)? as u16; // Get low byte of data 
        cpu.sp += 1; // increment stack pointer
        let data_high = (bus.read_byte(cpu.sp)? as u16) << 8; // Get high byte of data
        cpu.sp += 1; // Increment stack pointer

        // get full 16-bit data
        let data = data_high | data_low;

        cpu.pc = data;
        cpu.ime = true;

        Ok(0)
    }
}
