mod system;

use std::{
    collections::VecDeque,
    env,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use rodio::{DeviceSinkBuilder, Source, nz};
use system::System;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
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

struct ApuSource {
    buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl Iterator for ApuSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut buffer = self.buffer.lock().unwrap();
        let sample = buffer.pop_front();

        Some(sample.unwrap_or(0.0))
    }
}

impl Source for ApuSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        nz!(1)
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        nz!(44100)
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl App {
    fn new(rom_fname: &str) -> Self {
        let system = System::new(rom_fname).unwrap_or_else(|e| panic!("{e}"));

        let stream_handle =
            DeviceSinkBuilder::open_default_sink().expect("Open default audio stream");

        let audio_source = ApuSource {
            buffer: system.get_apu_buffer(),
        };

        stream_handle.mixer().add(audio_source);

        let _ = Box::leak(Box::new(stream_handle));

        Self {
            window: None,
            pixels: None,
            system,
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
            WindowEvent::KeyboardInput { event, .. } => {
                if event.repeat {
                    return;
                }

                if event.logical_key == Key::Named(NamedKey::Escape) {
                    event_loop.exit();
                }

                let key_index = match event.logical_key.as_ref() {
                    Key::Named(NamedKey::ArrowRight) => 0,
                    Key::Named(NamedKey::ArrowLeft) => 1,
                    Key::Named(NamedKey::ArrowUp) => 2,
                    Key::Named(NamedKey::ArrowDown) => 3,
                    Key::Character("x") => 4,
                    Key::Character("z") => 5,
                    Key::Character("a") => 6,
                    Key::Character("s") => 7,
                    _ => {
                        return;
                    }
                };
                let val = match event.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                self.system.change_key(key_index, val);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let logic_instant = Instant::now();
        let mut total_cycles_elapsed = 0;
        let mut cycles_elapsed = 0;
        for _ in 0..SPEED_UP {
            while cycles_elapsed < 70224 {
                let steps = self
                    .system
                    .step_cpu()
                    .unwrap_or_else(|e| panic!("Failed to step CPU: {e}"));

                self.system
                    .tick_subsystems(steps)
                    .unwrap_or_else(|e| panic!("Failed to tick subSystems: {e}"));
                cycles_elapsed += steps as u32;
            }
            total_cycles_elapsed += cycles_elapsed;
            cycles_elapsed = 0;
        }
        let logic_time = logic_instant.elapsed();

        let elapsed = self.last_frame.elapsed();
        let target_frame_time = Duration::from_secs_f64(70224.0 / 4_194_304.0);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed - Duration::from_micros(100));
        }
        let frame_time = self.last_frame.elapsed();

        println!(
            "logic_time: {:?}, frame time: {:?}, cycles_elapsed: {}",
            logic_time, frame_time, total_cycles_elapsed
        );

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
