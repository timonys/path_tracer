use crate::path_tracer::path_trace_components::AccumulatedSampleBufferComponent;
pub use crate::renderer::render_components::FramebufferComponent;
use glam::Vec3;

pub fn render(
    framebuffer: &mut FramebufferComponent,
    accumulated_sample_buffer: &AccumulatedSampleBufferComponent,
) {
    // Ensure the sizes of the framebuffer and sample buffer match
    if framebuffer.buffer.len() != (framebuffer.width * framebuffer.height) {
        eprintln!(
            "Error: Framebuffer buffer size dont match! Expected {} but got {}",
            framebuffer.width * framebuffer.height,
            framebuffer.buffer.len()
        );
        return;
    }

    if accumulated_sample_buffer.sample_data.len() != (framebuffer.width * framebuffer.height) {
        eprintln!(
            "Error: Sample buffer size dont match! Expected {} but got {}",
            framebuffer.width * framebuffer.height,
            accumulated_sample_buffer.sample_data.len()
        );
        return;
    }

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let buffer_index = y * framebuffer.width + x;
            framebuffer.buffer[buffer_index] =
                convert_to_pixel(accumulated_sample_buffer.sample_data[buffer_index]);
        }
    }
}

fn convert_to_pixel(color: Vec3) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;

    // Combine RGB into a single u32 (assuming ARGB format, with A = 255)
    (255 << 24) | (r << 16) | (g << 8) | b
}
