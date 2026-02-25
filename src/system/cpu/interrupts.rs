use crate::system::{bus::Bus, cpu::CpuError};

impl super::Cpu {
    pub(super) fn service_interrupts(&mut self, bus: &mut Bus) -> Result<(), CpuError> {
        let interrupt_flags = bus.read_byte(0xFF0F)?;
        self.ime = false;

        let addr = if interrupt_flags & 0b00001 > 0 {
            // VBlank Interrupt -> 0x0040
            bus.write_byte(0xFF0F, interrupt_flags & 0b11110)?;
            0x0040
        } else if interrupt_flags & 0b00010 > 0 {
            // LCD Interrupt -> 0x0048
            bus.write_byte(0xFF0F, interrupt_flags & 0b11101)?;
            0x0048
        } else if interrupt_flags & 0b00100 > 0 {
            // Timer Interrupt -> 0x0050
            bus.write_byte(0xFF0F, interrupt_flags & 0b11011)?;
            0x0050
        } else if interrupt_flags & 0b01000 > 0 {
            // Serial Interrupt -> 0x0058
            bus.write_byte(0xFF0F, interrupt_flags & 0b10111)?;
            0x0058
        } else if interrupt_flags & 0b10000 > 0 {
            // Joypad Interrupt -> 0x0060
            bus.write_byte(0xFF0F, interrupt_flags & 0b01111)?;
            0x0060
        } else {
            return Err(CpuError::InterruptError(interrupt_flags));
        };

        let current_addr = self.pc;
        let current_addr_low = current_addr as u8;
        let current_addr_high = ((current_addr & 0xFF00) >> 8) as u8;

        self.sp -= 1;
        bus.write_byte(self.sp, current_addr_high)?;
        self.sp -= 1;
        bus.write_byte(self.sp, current_addr_low)?;

        self.pc = addr;

        Ok(())
    }
}
