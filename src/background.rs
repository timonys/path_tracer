use crate::path_tracer::path_tracer_structs::Ray;
use glam::Vec3;
use noiselib::prelude::*;

pub const HORIZON_COLOR: Vec3 = Vec3::new(1.0, 0.6, 0.3); // Orange near horizon
pub const ZENITH_COLOR: Vec3 = Vec3::new(0.2, 0.4, 0.8); // Deep blue at zenith

pub const SUN_DIRECTION: Vec3 = Vec3::new(1.0, 0.5, 0.0);
pub const SUN_BRIGHTNESS_THRESHOLD: f32 = 0.995;
pub const SUN_BRIGHTNESS_MULTIPLIER: f32 = 100.0;

pub const CLOUD_SCALE: f32 = 5.0;
pub const CLOUD_SOFTNESS: f32 = 3.0;
pub const CLOUD_COLOR: Vec3 = Vec3::new(0.9, 0.9, 0.9);

pub fn create_background_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0); // Blend based on y direction

    // Base sky gradient (from horizon to zenith)
    let sky_color = (1.0 - t) * HORIZON_COLOR + t * ZENITH_COLOR;

    // Add the sun
    let sun_color = add_sun(unit_direction);

    // Add clouds
    let cloud_color = add_clouds(unit_direction);

    // Combine sky, sun, and clouds
    let combined_color = sky_color + sun_color + cloud_color;

    // Clamp the color to stay in [0.0, 1.0]
    combined_color.min(Vec3::ONE)
}

// Simulate a bright sun at a fixed direction
fn add_sun(direction: Vec3) -> Vec3 {
    let dot = direction.dot(SUN_DIRECTION.normalize());

    // Adjust intensity based on how close to the sun's direction
    let intensity = (dot - SUN_BRIGHTNESS_THRESHOLD).max(0.0) * SUN_BRIGHTNESS_MULTIPLIER;
    Vec3::new(1.0, 0.9, 0.7) * intensity // Bright yellow/white sun
}

// Add clouds using Perlin noise
fn add_clouds(direction: Vec3) -> Vec3 {
    let seed = 1;
    let mut rng = UniformRandomGen::new(seed);

    let noise_val = perlin_noise_3d(
        &mut rng,
        direction.x * CLOUD_SCALE,
        direction.y * CLOUD_SCALE,
        direction.z * CLOUD_SCALE,
        seed,
    );

    // Normalize noise to [0.0, 1.0]
    let intensity = ((noise_val + 1.0) * 0.5).powf(CLOUD_SOFTNESS); // Higher power for soft edges
    CLOUD_COLOR * intensity
}
