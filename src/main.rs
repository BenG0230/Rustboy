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
const TILE_MAP_HEIGHT: usize = 513;

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

    let mut main_window = Window::new(MAIN_WIDTH, MAIN_HEIGHT, 4);
    main_window.set_target_fps(60);

    let mut tile_map_window = Window::new(TILE_MAP_WIDTH, TILE_MAP_HEIGHT, 2);
    let mut temp_map_buffer = vec![0xFF00FF; TILE_MAP_WIDTH * TILE_MAP_HEIGHT];

    // let mut tiles_window = Window::new(TILES_WIDTH, TILES_HEIGHT, 4);
    // let mut temp_tile_buffer = vec![0; TILES_WIDTH * TILES_HEIGHT];

    let mut last_frame = Instant::now();

    let mut cycles_elapsed = 0;
    while main_window.is_open() && !main_window.is_key_down(Key::Escape) {
        // Limit systems to 70224 t-cycles per frame
        // while cycles_elapsed < 70224 {
        let steps = system
            .step_cpu()
            .unwrap_or_else(|e| panic!("Failed to step CPU: {e}"));

        // Tick other subSystems by number of t-cycles
        system
            .tick_subsystems(steps)
            .unwrap_or_else(|e| panic!("Failed to tick subSystems: {e}"));

        cycles_elapsed += steps as u32;
        // }

        // Update window
        if system.vblank {
            // if cycles_elapsed > 7000 {
            let elapsed = last_frame.elapsed();
            main_window.update(system.get_frame_buffer());

            system.render_tile_maps(&mut temp_map_buffer);
            tile_map_window.update(&mut temp_map_buffer);

            // system.render_tile_banks(&mut temp_tile_buffer);
            // tiles_window.update(&mut temp_tile_buffer);

            println!(
                "{:?} - {:?} - {}",
                elapsed,
                last_frame.elapsed(),
                cycles_elapsed
            );

            last_frame = Instant::now();
            system.vblank = false;
            cycles_elapsed = 0;
        }
    }
}
