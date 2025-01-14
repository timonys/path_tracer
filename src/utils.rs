use glam::{Vec2, Vec3};
use rand::Rng;

pub const RANDOM_VECTOR_MIN: f32 = -0.5;
pub const RANDOM_VECTOR_MAX: f32 = 0.5;

fn rand_in_range(min: f32, max: f32, rng: &mut impl Rng) -> f32 {
    rng.gen_range(min..max)
}

#[allow(dead_code)]
pub fn rand_vec2() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(RANDOM_VECTOR_MIN..RANDOM_VECTOR_MAX);
    let y = rng.gen_range(RANDOM_VECTOR_MIN..RANDOM_VECTOR_MAX);
    Vec2::new(x, y)
}

pub fn rand_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rand_in_range(RANDOM_VECTOR_MIN, RANDOM_VECTOR_MAX, &mut rng);
    let y = rand_in_range(RANDOM_VECTOR_MIN, RANDOM_VECTOR_MAX, &mut rng);
    let z = rand_in_range(RANDOM_VECTOR_MIN, RANDOM_VECTOR_MAX, &mut rng);
    Vec3::new(x, y, z)
}

#[allow(dead_code)]
pub fn rand_vec2_between(min: f32, max: f32) -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rand_in_range(min, max, &mut rng);
    let y = rand_in_range(min, max, &mut rng);
    Vec2::new(x, y)
}

pub fn rand_vec3_between(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rand_in_range(min, max, &mut rng);
    let y = rand_in_range(min, max, &mut rng);
    let z = rand_in_range(min, max, &mut rng);
    Vec3::new(x, y, z)
}

pub fn is_near_zero_length(vec: Vec3, epsilon: f32) -> bool {
    vec.length_squared() < epsilon * epsilon
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn gamma_correct_color(color: &mut Vec3) {
    color.x = linear_to_gamma(color.x);
    color.y = linear_to_gamma(color.y);
    color.z = linear_to_gamma(color.z);
}

pub fn schlick_reflectance(cos: f32, refraction_ratio: f32) -> f32 {
    // Schlick's approximation for reflectance
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
