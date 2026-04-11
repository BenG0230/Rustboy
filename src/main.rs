mod system;

use std::{
    env,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use system::System;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SPEED_UP: u32 = 1;

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    system: System,
    last_frame: Instant,
}

impl App {
    fn new(rom_fname: &str) -> Self {
        Self {
            window: None,
            pixels: None,
            system: System::new(rom_fname).unwrap_or_else(|e| panic!("{e}")),
            last_frame: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new((WIDTH * 4) as f64, (HEIGHT * 4) as f64);
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Rustboy Emulator")
                    .with_inner_size(size),
            )
            .unwrap();
        let window_ref: &'static Window = Box::leak(Box::new(window));

        let surface = SurfaceTexture::new(size.width as u32, size.height as u32, window_ref);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        self.window = Some(window_ref);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    self.system.copy_frame_buffer(frame);

                    pixels.render().unwrap();
                }

                if let Some(window) = &mut self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let logic_instant = Instant::now();
        for _ in 0..SPEED_UP {
            let mut cycles_elapsed = 0;
            while !self.system.vblank {
                let steps = self
                    .system
                    .step_cpu()
                    .unwrap_or_else(|e| panic!("Failed to step CPU: {e}"));

                self.system
                    .tick_subsystems(steps)
                    .unwrap_or_else(|e| panic!("Failed to tick subSystems: {e}"));
                cycles_elapsed += 1;
            }

            self.system.vblank = false;
        }
        let logic_time = logic_instant.elapsed();

        self.window.as_ref().unwrap().request_redraw();

        let elapsed = self.last_frame.elapsed();
        let target_frame_time = Duration::from_secs_f64(1.0 / 59.73);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
        let frame_time = self.last_frame.elapsed();

        println!("logic_time: {:?}, frame time: {:?}", logic_time, frame_time);

        self.last_frame = Instant::now();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("No Args given\n Needs filename!");
        return Ok(());
    }

    let rom_fname = &args[1];

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(rom_fname);
    event_loop.run_app(&mut app)?;

    Ok(())
}
