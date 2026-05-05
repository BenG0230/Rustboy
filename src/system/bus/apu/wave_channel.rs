use crate::system::bus::BusError;

pub struct WaveChannel {
    pub enabled: bool,
    dac_enabled: bool,
    period: u16,
    period_timer: u16,

    len_enabled: bool,
    initial_len: u8,
    len_timer: u16,
    internal_len_timer: u32,

    volume: u8,

    wave_ram: [u8; 16],
    wave_index: u8,
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            period: 0x7FF,
            period_timer: 0x7FF,
            len_enabled: false,
            initial_len: 0,
            len_timer: 0,
            internal_len_timer: 0,
            volume: 0,
            wave_ram: [0; 16],
            wave_index: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF1A => {
                let mut output = 0b01111111;
                output |= (self.dac_enabled as u8) << 7;
                Ok(output)
            }
            0xFF1B => Ok(self.initial_len),
            0xFF1C => {
                let mut output = 0b10011111;
                output |= (self.volume & 0b11) << 5;
                Ok(output)
            }
            0xFF1E => {
                let mut output = 0b10111111;
                output |= (self.len_enabled as u8) << 6;
                Ok(output)
            }
            0xFF30..=0xFF3F => Ok(self.wave_ram[(addr - 0xFF30) as usize]),
            _ => Ok(0xFF),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) -> Result<(), BusError> {
        match addr {
            0xFF1A => self.dac_enabled = (val & 0x80) > 0,
            0xFF1B => self.initial_len = val,
            0xFF1C => self.volume = (val & 0x60) >> 5,
            0xFF1D => self.period = (self.period & 0x700) | val as u16,
            0xFF1E => {
                self.period = (self.period & 0xFF) | ((val as u16 & 0b111) << 8);
                self.len_enabled = (val & 0x40) > 0;

                if val & 0x80 > 0 {
                    self.enabled = true;
                    if self.len_timer >= 256 {
                        self.len_timer = self.initial_len as u16;
                    }
                    self.internal_len_timer = 0;

                    self.period_timer = self.period;
                    self.wave_index = 0;
                }
            }
            0xFF30..=0xFF3F => self.wave_ram[(addr - 0xFF30) as usize] = val,
            _ => {}
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        self.period_timer += 1;

        if self.len_enabled {
            self.internal_len_timer += 1;
            if self.internal_len_timer >= 4096 {
                self.len_timer += 1;
                if self.len_timer >= 256 {
                    self.len_timer = self.initial_len as u16;
                    self.enabled = false;
                }
                self.internal_len_timer = 0;
            }
        }

        if self.period_timer >= 2048 {
            self.period_timer = self.period;
            self.wave_index = (self.wave_index + 1) % 32;
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.dac_enabled || !self.enabled {
            return 0.0;
        }

        let upper = !(self.wave_index % 2 == 0);
        let byte = self.wave_ram[(self.wave_index / 2) as usize];

        let mut nibble = if upper {
            (byte & 0xF0) >> 4
        } else {
            byte & 0xF
        };

        let amplitude = match self.volume {
            0b00 => 4,
            0b01 => 0,
            0b10 => 1,
            0b11 => 2,
            _ => 4,
        };

        nibble = nibble >> amplitude;

        let sample: f32 = (nibble as f32 / 15.0) * 2.0 - 1.0;

        sample
    }
}
