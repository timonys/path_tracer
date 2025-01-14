use glam::{Vec2, Vec3};
use rand::Rng;

pub const RANDOM_VECTOR_MIN: f32 = -0.5;
pub const RANDOM_VECTOR_MAX: f32 = 0.5;

fn rand_in_range(min: f32, max: f32, rng: &mut impl Rng) -> f32 {
    assert!(
        min < max,
        "min must be less than max: min = {}, max = {}",
        min,
        max
    );

    rng.gen_range(min..max)
}

#[allow(dead_code)]
pub fn rand_vec2() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(RANDOM_VECTOR_MIN..RANDOM_VECTOR_MAX);
    let y = rng.gen_range(RANDOM_VECTOR_MIN..RANDOM_VECTOR_MAX);

    assert!(
        (RANDOM_VECTOR_MIN..=RANDOM_VECTOR_MAX).contains(&x),
        "Random X value out of range"
    );
    assert!(
        (RANDOM_VECTOR_MIN..=RANDOM_VECTOR_MAX).contains(&y),
        "Random Y value out of range"
    );

    Vec2::new(x, y)
}

pub fn rand_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rand_in_range(RANDOM_VECTOR_MIN, RANDOM_VECTOR_MAX, &mut rng);
    let y = rand_in_range(RANDOM_VECTOR_MIN, RANDOM_VECTOR_MAX, &mut rng);
    let z = rand_in_range(RANDOM_VECTOR_MIN, RANDOM_VECTOR_MAX, &mut rng);

    assert!(
        (RANDOM_VECTOR_MIN..=RANDOM_VECTOR_MAX).contains(&x),
        "Random X value out of range"
    );
    assert!(
        (RANDOM_VECTOR_MIN..=RANDOM_VECTOR_MAX).contains(&y),
        "Random Y value out of range"
    );
    assert!(
        (RANDOM_VECTOR_MIN..=RANDOM_VECTOR_MAX).contains(&z),
        "Random Z value out of range"
    );

    Vec3::new(x, y, z)
}

#[allow(dead_code)]
pub fn rand_vec2_between(min: f32, max: f32) -> Vec2 {
    assert!(
        min < max,
        "min must be less than max: min = {}, max = {}",
        min,
        max
    );

    let mut rng = rand::thread_rng();

    let x = rand_in_range(min, max, &mut rng);
    let y = rand_in_range(min, max, &mut rng);

    assert!(x >= min && x <= max, "Random X value out of range");
    assert!(y >= min && y <= max, "Random Y value out of range");

    Vec2::new(x, y)
}

pub fn rand_vec3_between(min: f32, max: f32) -> Vec3 {
    assert!(
        min < max,
        "min must be less than max: min = {}, max = {}",
        min,
        max
    );

    let mut rng = rand::thread_rng();

    let x = rand_in_range(min, max, &mut rng);
    let y = rand_in_range(min, max, &mut rng);
    let z = rand_in_range(min, max, &mut rng);

    assert!(x >= min && x <= max, "Random X value out of range");
    assert!(y >= min && y <= max, "Random Y value out of range");
    assert!(z >= min && z <= max, "Random Z value out of range");

    Vec3::new(x, y, z)
}

pub fn is_near_zero_length(vec: Vec3, epsilon: f32) -> bool {
    assert!(
        epsilon > 0.0,
        "Epsilon must be greater than zero: epsilon = {}",
        epsilon
    );

    vec.length_squared() < epsilon * epsilon
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    assert!(
        linear_component >= 0.0,
        "Linear component should be non-negative: linear_component = {}",
        linear_component
    );

    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn gamma_correct_color(color: &mut Vec3) {
    assert!(
        color.x >= 0.0 && color.y >= 0.0 && color.z >= 0.0,
        "Color components should be non-negative"
    );

    color.x = linear_to_gamma(color.x);
    color.y = linear_to_gamma(color.y);
    color.z = linear_to_gamma(color.z);
}

pub fn schlick_reflectance(cos: f32, refraction_ratio: f32) -> f32 {
    assert!(
        (-1.0..=1.0).contains(&cos),
        "Cosine value out of bounds: cos = {}",
        cos
    );

    assert!(
        refraction_ratio > 0.0,
        "Refraction ratio must be greater than zero: refraction_ratio = {}",
        refraction_ratio
    );

    // Schlick's approximation for reflectance
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
