mod system;

use std::env;
use system::System;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("No Args given\n Needs filename");
        return;
    }

    let rom_fname = &args[1];

    match System::new(rom_fname) {
        Ok(mut system) => match system.run() {
            Ok(_) => {}
            Err(e) => println!("Error: {}", e),
        },
        Err(e) => println!("Error: {}", e),
    }
}
