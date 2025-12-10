mod bus;
mod cpu;

use bus::Bus;
use cpu::Cpu;

pub struct System {
    cpu: Cpu,
    bus: Bus,
}

impl System {
    pub fn new(rom_fname: &str) -> Self {
        let mut bus = Bus::new();

        match bus.load_rom(rom_fname) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        Self {
            cpu: Cpu::new(),
            bus,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        //TODO: each loop call sub-systems steps
        //increment main system clock each loop according to cpu clock
        //Each sub-system "catches up" to main system clock
        loop {
            let steps = self.cpu.step(&mut self.bus);
            println!("steps: {}", steps);
        }
    }
}
