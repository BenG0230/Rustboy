use minifb::{Key, Window as minifbWindow, WindowOptions};

pub struct Window {
    window: minifbWindow,
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize, scale: u8) -> Self {
        let win_scale = match scale {
            1 => minifb::Scale::X1,
            2 => minifb::Scale::X2,
            4 => minifb::Scale::X4,
            8 => minifb::Scale::X8,
            16 => minifb::Scale::X16,
            32 => minifb::Scale::X32,
            _ => minifb::Scale::X1,
        };

        Self {
            window: minifbWindow::new(
                "Rustboy emulator",
                width,
                height,
                WindowOptions {
                    resize: false,
                    scale: win_scale,
                    topmost: true,
                    ..WindowOptions::default()
                },
            )
            .unwrap_or_else(|e| panic!("{e}")),
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}
