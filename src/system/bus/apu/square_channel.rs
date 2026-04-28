use crate::system::bus::BusError;

pub struct SquareChannel {
    enabled: bool,
    phase: f32,
    frequency: u16,    // Split between NR*3/4
    period_timer: u16, // Number of t-cycles till next step

    duty_index: u8, // 0-7 index position
    duty: u8,       // 0->0.125 1->0.25 2->0.5 3->0.75

    // Sweep stuff (channel 1 only!)
    enable_sweep: bool,
    sweep_step: u8,   // 3-bit step size
    sweep_dir: bool,  // true -> increase, false -> decrease
    sweep_period: u8, // sweep updated every period*7.8ms
    sweep_timer: u8,

    len_enabled: bool,
    initial_len: u8,
    len_timer: u8,

    volume: u8,
    initial_vol: u8,
    envelope_dir: bool,  // true -> increase, false -> decrease
    envelope_period: u8, // envelope updated every period*64Hz
    envelope_timer: u8,
}

/*
 * ### register mapping ###
 *         Bit(s)     |     Thing
 *         ------------------------------------
 * NR10 -> 0-2        |  sweep step
 *         3          |  sweep direction
 *         4-6        |  sweep period
 *                    |
 * NR*1 -> 0-5        |  Initial length timer
 *         6-7        |  Wave duty
 *                    |
 * NR*2 -> 0-2        |   Envelope period
 *         3          |  Envelope direction
 *         4-7        |   Initial volume
 *                    |
 * NR*3 -> 0-7        |   lower 8 bits of frequency
 *                    |
 * NR*4 -> 0-2        |    upper 3 bits of frequency
 *         6          |    Length enabled
 *         7          |    Trigger
 */

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
            phase: 0.0,
            frequency: 0x7FF,
            period_timer: 0x7FF,
            duty_index: 0,
            duty: 0b00,
            enable_sweep,
            sweep_step: 0,
            sweep_dir: false,
            sweep_period: 0,
            sweep_timer: 0,
            len_enabled: false,
            initial_len: 0,
            len_timer: 0,
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
            0xFF11 => {
                let mut output = 0;
                output |= self.initial_len & 0b111111;
                output |= (self.duty & 0b11) << 6;
                Ok(output)
            }
            0xFF12 => {
                let mut output = 0;
                output |= self.envelope_period & 0b111;
                output |= (self.envelope_dir as u8) << 3;
                output |= (self.initial_vol & 0b1111) << 4;

                Ok(output)
            }
            0xFF13 => Ok((self.frequency & 0xFF) as u8),
            0xFF14 => {
                let mut output = 0b10111000;
                output |= ((self.frequency & 0x700) >> 8) as u8;
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
                self.duty = (val & 0b11000000) >> 6;
            }
            0xFF12 | 0xFF17 => {
                self.envelope_period = val & 0b111;
                self.envelope_dir = (val & 0b1000) > 0;
                self.initial_vol = (val & 0b11110000) >> 4;
            }
            0xFF13 | 0xFF18 => self.frequency = (self.frequency & 0x700) | val as u16,
            0xFF14 | 0xFF19 => {
                self.frequency = (self.frequency & 0xFF) | ((val as u16 & 0b11) << 8);
                self.len_enabled = (val & 0b1000000) > 0;

                if val & 0x80 > 0 {
                    self.enabled = true;
                    self.len_timer = self.initial_len;
                    self.period_timer = (2048 - self.frequency) * 4;
                    self.envelope_timer = 0;
                    self.volume = self.initial_vol;
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
        //
        // self.period_timer -= 1;
        //
        // if self.period_timer == 0 {
        //     // step once in timer
        //
        //     self.period_timer = (2048 - self.period_val) * 4; // reset timer
        //     self.duty_index = (self.duty_index + 1) % 8; // keep it [0,8]
        // }
    }

    pub fn sample(&mut self) -> f32 {
        if !self.enabled || self.volume == 0 {
            return 0.0;
        }

        let duty_index = (self.phase * 8.0) as usize % 8;
        let bit = get_duty_cycle_from_int(self.duty)[duty_index];

        let wave = if bit == 1 { 1.0 } else { 0.0 };

        let amplitude = self.volume as f32 / 15.0;

        let freq = 131072.0 / (2048 - self.frequency) as f32;
        self.phase += freq / 44100.0;

        wave * amplitude
    }
}
