use crate::system::bus::BusError;

pub struct Timers {
    // --- Internal Counter ---
    counter: u16,
    prev_counter: u16,

    // --- Registers ---
    // DIV derived from counter >> 8
    tima: u8,
    tma: u8,
    tac: u8,

    // --- Overflow logic ---
    overflow: bool,
    overflow_count: u8,

    // --- Flags ---
    requesting_interrupt: bool,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            counter: 0xAB00,
            prev_counter: 0xAB00,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
            overflow: false,
            overflow_count: 0,
            requesting_interrupt: false,
        }
    }

    pub(super) fn tick(&mut self) {
        if self.overflow {
            if self.overflow_count == 0 {
                self.overflow = false;
                self.tima = self.tma;
                self.requesting_interrupt = true;
            } else {
                self.overflow_count -= 1;
            }
        }

        self.prev_counter = self.counter;
        self.counter = self.counter.wrapping_add(1);

        self.check_timer(); // check for falling edge
    }

    pub(super) fn read_reg(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF04 => Ok((self.counter >> 8) as u8),
            0xFF05 => Ok(self.tima),
            0xFF06 => Ok(self.tma),
            0xFF07 => Ok(self.tac),
            _ => Err(BusError::OutOfRange(addr)),
        }
    }

    pub(super) fn write_reg(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF04 => {
                self.prev_counter = self.counter;
                self.counter = 0;
                self.check_timer(); // check for falling edge

                Ok(())
            }
            0xFF05 => {
                if self.overflow {
                    self.overflow = false;
                    self.overflow_count = 0;
                }

                self.tima = val;
                Ok(())
            }
            0xFF06 => {
                self.tma = val;
                Ok(())
            }
            0xFF07 => {
                // If changing enabled tick timer if bit 1
                if self.tac & 0b100 > 0 && val & 0b100 == 0 {
                    let timer_bit = self.get_timer_bit();

                    if (self.counter >> timer_bit) & 1 == 1 {
                        self.tick_tima();
                    }
                }

                // If changing timer bit (bit 1,0) check for timer tick
                if self.tac & 0b11 != val & 0b11 && self.timer_enabled() {
                    let timer_bit_old = self.get_timer_bit();
                    let timer_bit_new = match val & 0b11 {
                        0 => 9,
                        1 => 3,
                        2 => 5,
                        3 => 7,
                        _ => unreachable!(),
                    };

                    let prev_bit = (self.counter >> timer_bit_old) & 1;
                    let new_bit = (self.counter >> timer_bit_new) & 1;

                    if prev_bit == 1 && new_bit == 0 {
                        self.tick_tima();
                    }
                }
                self.tac = val;
                Ok(())
            }
            _ => Err(BusError::OutOfRange(addr)),
        }
    }

    pub(super) fn check_for_interrupt(&mut self) -> bool {
        if self.requesting_interrupt {
            self.requesting_interrupt = false;
            true
        } else {
            false
        }
    }

    fn check_timer(&mut self) {
        if self.timer_enabled() {
            let timer_bit = self.get_timer_bit();

            let prev_bit = (self.prev_counter >> timer_bit) & 1;
            let new_bit = (self.counter >> timer_bit) & 1;

            if prev_bit == 1 && new_bit == 0 {
                self.tick_tima();
            }
        }
    }

    fn tick_tima(&mut self) {
        self.tima = self.tima.wrapping_add(1);
        if self.tima == 0 {
            self.overflow = true;
            self.overflow_count = 4;
        }
    }

    fn get_timer_bit(&self) -> u8 {
        match self.tac & 0b11 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        }
    }

    fn timer_enabled(&self) -> bool {
        (self.tac & 0b100) > 0
    }
}
