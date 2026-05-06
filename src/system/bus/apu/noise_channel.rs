use crate::system::bus::BusError;

pub struct NoiseChannel {
    pub enabled: bool,
    period_timer: u16,

    clock_shift: u8,
    lfsr_width: bool, // false -> 15-bit true -> 7bit
    clock_div: u8,
    lfsr: u16,

    len_enabled: bool,
    initial_len: u8,
    len_timer: u8,
    internal_len_timer: u32,

    volume: u8,
    initial_vol: u8,
    envelope_dir: bool,
    envelope_period: u8,
    envelope_timer: u32,
}

static DIVISORS: [u8; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            period_timer: 0,
            clock_shift: 0,
            lfsr_width: false,
            clock_div: 0,
            lfsr: 0,
            len_enabled: false,
            initial_len: 0,
            len_timer: 0,
            internal_len_timer: 0,
            volume: 0,
            initial_vol: 0,
            envelope_dir: false,
            envelope_period: 0,
            envelope_timer: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF21 => {
                let mut output = 0;
                output |= self.envelope_period & 0b111;
                output |= (self.envelope_dir as u8) << 3;
                output |= (self.initial_vol & 0b1111) << 4;

                Ok(output)
            }
            0xFF22 => {
                let mut output = 0;
                output |= self.clock_div & 0b111;
                output |= (self.lfsr_width as u8) << 3;
                output |= (self.clock_shift * 0b1111) << 4;

                Ok(output)
            }
            0xFF23 => {
                let mut output = 0b10111111;
                output |= (self.len_enabled as u8) << 6;

                Ok(output)
            }
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF20 => self.initial_len = val & 0x3F,
            0xFF21 => {
                self.envelope_period = val & 0b111;
                self.envelope_dir = (val & 0b1000) > 0;
                self.initial_vol = (val & 0b11110000) >> 4;

                if (val & 0b11111000) == 0 {
                    self.volume = 0;
                }
            }
            0xFF22 => {
                self.clock_div = val & 0b111;
                self.lfsr_width = (val & 0b1000) > 0;
                self.clock_shift = (val & 0b11110000) >> 4;
            }
            0xFF23 => {
                self.len_enabled = (val & 0b1000000) > 0;

                if val & 0x80 > 0 {
                    self.enabled = true;
                    if self.len_timer >= 64 {
                        self.len_timer = self.initial_len;
                    }
                    self.internal_len_timer = 0;
                    self.period_timer =
                        (DIVISORS[self.clock_div as usize] << self.clock_shift) as u16;
                    self.envelope_timer = 0;
                    self.volume = self.initial_vol;
                    self.lfsr = 0;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        if self.period_timer != 0 {
            self.period_timer -= 1;
        }

        // Envelope
        if self.envelope_period != 0 {
            self.envelope_timer += 1;
            if self.envelope_timer >= 65536 * self.envelope_period as u32 {
                if self.envelope_dir && self.volume < 15 {
                    self.volume += 1;
                } else if !self.envelope_dir && self.volume > 0 {
                    self.volume -= 1;
                }

                self.envelope_timer = 0;
            }
        }

        // Length
        if self.len_enabled {
            self.internal_len_timer += 1;
            if self.internal_len_timer >= 16384 {
                self.len_timer += 1;
                if self.len_timer >= 64 {
                    self.len_timer = self.initial_len;
                    self.enabled = false;
                }
                self.internal_len_timer = 0;
            }
        }

        if self.period_timer == 0 {
            self.period_timer = (DIVISORS[self.clock_div as usize] << self.clock_shift) as u16;

            // tick lfsr
            let new_bit = !((self.lfsr & 0b1) ^ ((self.lfsr >> 1) & 0b1));
            self.lfsr = (self.lfsr & 0x7FFF) | (new_bit << 15);
            if self.lfsr_width {
                self.lfsr = (self.lfsr & 0xFF7F) | (new_bit << 7);
            }
            self.lfsr >>= 1;
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let sample = if self.lfsr & 1 > 0 { 1.0 } else { -1.0 };

        let amplitude = self.volume as f32 / 15.0;

        sample * amplitude
    }
}
