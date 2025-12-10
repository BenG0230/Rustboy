mod system;

use std::env;
use system::System;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("No Args given\n Needs filename");
        return Ok(());
    }

    let rom_fname = &args[1];
    let system_result = System::new(rom_fname).run();

    system_result
}
