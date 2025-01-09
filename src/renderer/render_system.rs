pub use crate::renderer::render_components::*;
use rand::Rng;

pub fn render(framebuffer: &mut FramebufferComponent) {
    let random_color_u32 = generate_random_color_u32();
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let buffer_index = y * framebuffer.width + x;
            framebuffer.buffer[buffer_index] = random_color_u32;
        }
    }
}

fn generate_random_color_u32() -> u32 {
    let mut rng = rand::thread_rng();

    // Generate random values for red, green, and blue (0 to 255)
    let red = rng.gen_range(0..=255);
    let green = rng.gen_range(0..=255);
    let blue = rng.gen_range(0..=255);

    // Combine the RGB components into a u32 value
    // The format is 0xRRGGBB, where the red, green, and blue are placed in the correct byte positions
    (red << 16) | (green << 8) | blue
}