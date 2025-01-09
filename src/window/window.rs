use minifb::{Window, WindowOptions};
pub struct WindowWrapper {
    window: Window,
}

impl WindowWrapper {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(title, width, height, WindowOptions::default())
            .expect("Failed to build window");

        Self { window }
    }

    pub fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn update_window(&mut self, buffer: &[u32]) {
        self.window
            .update_with_buffer(
                buffer,
                buffer.len() / self.window.get_size().0,
                buffer.len() / self.window.get_size().1,
            )
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        return self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape);
    }
}
