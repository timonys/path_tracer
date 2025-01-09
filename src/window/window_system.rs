pub use crate::window::window_component::*;
use crate::FramebufferComponent;
use minifb::{Window, WindowOptions};

pub fn run_window(window: &mut WindowComponent, framebuffer: &FramebufferComponent) {
    if window.handle.is_none() {
        //If no window handle, create a window
        let minifb_window = Window::new(
            window.title.as_str(),
            window.width,
            window.height,
            WindowOptions::default(),
        )
        .expect("Failed to build window");

        window.handle = Some(minifb_window);
    }

    if let Some(handle) = &mut window.handle {
        if !handle.is_open() || handle.is_key_down(minifb::Key::Escape) {
            std::process::exit(0);
        }

        handle
            .update_with_buffer(&framebuffer.buffer, window.width, window.height)
            .expect("Failed to update the window buffer");
    }
}
