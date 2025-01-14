use crate::renderer::render_components::FramebufferComponent;
use crate::window::window_component::*;

pub fn update_window(window: &mut WindowComponent, framebuffer: &FramebufferComponent) {
    if !window.handle.is_open() || window.handle.is_key_down(minifb::Key::Escape) {
        std::process::exit(0);
    }

    let (width, height) = window.handle.get_size();
    window
        .handle
        .update_with_buffer(&framebuffer.buffer, width, height)
        .expect("Failed to update the window buffer");
}
