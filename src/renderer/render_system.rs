pub use crate::renderer::render_components::*;
use crate::AccumulatedSampleBufferComponent;
use flecs_ecs::prelude::*;
use glam::Vec3;

pub fn render(e: EntityView, framebuffer: &mut FramebufferComponent) {
    let query = e.world().new_query::<(&AccumulatedSampleBufferComponent)>();

    query.each(|buffer| {
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                let buffer_index = y * framebuffer.width + x;
                framebuffer.buffer[buffer_index] =
                    convert_to_pixel(buffer.sample_data[buffer_index]);
            }
        }
    });
}

fn convert_to_pixel(color: Vec3) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;

    // Combine RGB into a single u32 (assuming ARGB format, with A = 255)
    (255 << 24) | (r << 16) | (g << 8) | b
}
