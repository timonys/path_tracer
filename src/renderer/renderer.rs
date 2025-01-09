use crate::path_tracer::window::WindowWrapper;
pub struct Renderer {
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn render(&self, buffer: &[u32], window: &mut WindowWrapper) {
        window.update_window(buffer);
    }
}

