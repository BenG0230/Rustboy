use crate::system::bus::BusError;

pub struct SquareChannel {
    pub enabled: bool,
    period: u16,       // Split between NR*3/4
    period_timer: u16, // Number of t-cycles till next step

    duty_index: u8, // 0-7 index position
    duty_cycle: u8, // 0->0.125 1->0.25 2->0.5 3->0.75

    // Sweep stuff (channel 1 only!)
    enable_sweep: bool, // if channel 1
    sweep_step: u8,     // 3-bit step size
    sweep_dir: bool,    // false -> increase, true -> decrease
    sweep_period: u8,   // sweep updated every period*7.8ms
    sweep_timer: u32,

    len_enabled: bool,
    initial_len: u8,
    len_timer: u8,
    internal_len_timer: u32,

    volume: u8,
    initial_vol: u8,
    envelope_dir: bool,  // true -> increase, false -> decrease
    envelope_period: u8, // envelope updated every period*64Hz
    envelope_timer: u32,
}

fn get_duty_cycle_from_int(duty_int: u8) -> [u8; 8] {
    match duty_int {
        0 => [0, 0, 0, 0, 0, 0, 0, 1],
        1 => [1, 0, 0, 0, 0, 0, 0, 1],
        2 => [1, 0, 0, 0, 0, 1, 1, 1],
        3 => [0, 1, 1, 1, 1, 1, 1, 0],
        _ => [0, 0, 0, 0, 0, 0, 0, 0],
    }
}

impl SquareChannel {
    pub fn new(enable_sweep: bool) -> Self {
        Self {
            enabled: false,
            period: 0x7FF,
            period_timer: 0x7FF,
            duty_index: 0,
            duty_cycle: 0,
            enable_sweep,
            sweep_step: 0,
            sweep_dir: false,
            sweep_period: 0,
            sweep_timer: 0,
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
            0xFF10 => {
                let mut output = 0b10000000;
                output |= self.sweep_step & 0b111;
                output |= (self.sweep_dir as u8) << 3;
                output |= (self.sweep_period & 0b111) << 4;

                Ok(output)
            }
            0xFF11 | 0xFF16 => {
                let mut output = 0;
                output |= self.initial_len & 0b111111;
                output |= (self.duty_cycle & 0b11) << 6;
                Ok(output)
            }
            0xFF12 | 0xFF17 => {
                let mut output = 0;
                output |= self.envelope_period & 0b111;
                output |= (self.envelope_dir as u8) << 3;
                output |= (self.initial_vol & 0b1111) << 4;

                Ok(output)
            }
            0xFF14 | 0xFf19 => {
                let mut output = 0b10111111;
                output |= (self.len_enabled as u8) << 6;

                Ok(output)
            }
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF10 => {
                self.sweep_step = val & 0b111;
                self.sweep_dir = (val & 0b1000) > 0;
                self.sweep_period = (val & 0b1110000) >> 4;
            }
            0xFF11 | 0xFF16 => {
                self.initial_len = val & 0b111111;
                self.duty_cycle = (val & 0b11000000) >> 6;
            }
            0xFF12 | 0xFF17 => {
                self.envelope_period = val & 0b111;
                self.envelope_dir = (val & 0b1000) > 0;
                self.initial_vol = (val & 0b11110000) >> 4;

                if (val & 0b11111000) == 0 {
                    self.enabled = false;
                }
            }
            0xFF13 | 0xFF18 => self.period = (self.period & 0x700) | val as u16,
            0xFF14 | 0xFF19 => {
                self.period = (self.period & 0xFF) | ((val as u16 & 0b111) << 8);
                self.len_enabled = (val & 0b1000000) > 0;

                if val & 0x80 > 0 {
                    self.enabled = true;
                    if self.len_timer >= 64 {
                        self.len_timer = self.initial_len;
                    }
                    self.internal_len_timer = 0;
                    self.period_timer = self.period;
                    self.envelope_timer = 0;
                    self.volume = self.initial_vol;

                    self.sweep_timer = 0;
                }
            }
            _ => return Ok(()),
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        self.period_timer += 1;

        // Envelope
        if self.envelope_period != 0 {
            self.envelope_timer += 1;
            if self.envelope_timer >= 16384 * self.envelope_period as u32 {
                if self.envelope_dir && self.volume < 15 {
                    self.volume += 1;
                } else if !self.envelope_dir && self.volume > 0 {
                    self.volume -= 1;
                }

                self.envelope_timer = 0;
            }
        }

        // Sweep
        if self.sweep_period != 0 && self.sweep_step != 0 && self.enable_sweep {
            self.sweep_timer += 1;
            if self.sweep_timer >= 8192 * self.sweep_period as u32 {
                let base: u8 = 2; // required to use .pow()
                if !self.sweep_dir {
                    self.period += self.period / base.pow(self.sweep_step as u32) as u16;
                } else {
                    self.period -= self.period / base.pow(self.sweep_step as u32) as u16;
                }

                if self.period >= 2048 {
                    self.enabled = false;
                }
                self.sweep_timer = 0;
            }
        }

        // Length
        if self.len_enabled {
            self.internal_len_timer += 1;
            if self.internal_len_timer >= 4096 {
                self.len_timer += 1;
                if self.len_timer >= 64 {
                    self.len_timer = self.initial_len;
                    self.enabled = false;
                }
                self.internal_len_timer = 0;
            }
        }

        // Frequency/wave
        if self.period_timer >= 2048 {
            self.period_timer = self.period;
            self.duty_index = (self.duty_index + 1) % 8;
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let duty_wave = get_duty_cycle_from_int(self.duty_cycle);
        let bit = duty_wave[self.duty_index as usize];

        let sample = if bit == 1 { 1.0 } else { -1.0 };

        let amplitude = self.volume as f32 / 15.0;

        sample * amplitude
    }
}
