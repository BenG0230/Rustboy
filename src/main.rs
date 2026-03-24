mod system;
mod window;

use minifb::Key;
use std::env;
use std::time::Duration;
use std::time::Instant;
use system::System;
use window::Window;

const TILES_WIDTH: usize = 128;
const TILES_HEIGHT: usize = 192;

const TILE_MAP_WIDTH: usize = 256;
const TILE_MAP_HEIGHT: usize = 512;

const MAIN_WIDTH: usize = 160;
const MAIN_HEIGHT: usize = 144;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("No Args given\n Needs filename");
        return;
    }

    let rom_fname = &args[1];

    let mut system = System::new(rom_fname).unwrap_or_else(|e| panic!("{e}"));

    // let mut main_window = Window::new(MAIN_WIDTH, MAIN_HEIGHT, 4);
    // let mut tiles_window = Window::new(TILES_WIDTH, TILES_HEIGHT, 4);
    let mut tile_map_window = Window::new(TILE_MAP_WIDTH, TILE_MAP_HEIGHT, 2);

    let mut last_frame = Instant::now();
    // ~ 16.742ms
    let frame_duration = Duration::from_secs_f64(1.0 / 59.73);

    while tile_map_window.is_open() && !tile_map_window.is_key_down(Key::Escape) {
        // Limit systems to 70224 t-cycles per frame
        let mut cycles_elapsed = 0;
        while cycles_elapsed < 70224 {
            let steps = system
                .step_cpu()
                .unwrap_or_else(|e| panic!("Failed to step CPU: {e}"));

            // Tick other subSystems by number of t-cycles
            system
                .tick_subsystems(steps)
                .unwrap_or_else(|e| panic!("Failed to tick subSystems: {e}"));

            cycles_elapsed += steps as u32;
        }

        // Update window buffer from PPU
        // system.render_tile_banks(&mut tiles_window.buffer);
        // tiles_window.update();

        system.render_tile_maps(&mut tile_map_window.buffer);
        tile_map_window.update();

        // Check input

        // Limit frame rate to 59.73Hz
        let elapsed = last_frame.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        // println!("{:?}", last_frame.elapsed());
        last_frame = Instant::now();
        // std::thread::sleep(Duration::from_micros(1000));
    }
}
