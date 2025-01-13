use glam::{Vec2, Vec3};
use rand::Rng;

fn rand_in_range(min: f32, max: f32, rng: &mut impl Rng) -> f32 {
    rng.gen_range(min..max)
}

pub fn rand_vec2() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-0.5..0.5);
    let y = rng.gen_range(-0.5..0.5);
    Vec2::new(x, y)
}

pub fn rand_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rand_in_range(-0.5, 0.5, &mut rng);
    let y = rand_in_range(-0.5, 0.5, &mut rng);
    let z = rand_in_range(-0.5, 0.5, &mut rng);
    Vec3::new(x, y, z)
}

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

pub fn random_unit_vector() -> Vec3 {
    loop {
        // Generate a random vector within the cube [-1, 1] for each axis
        let p = rand_vec3_between(-1.0, 1.0);
        let length_sq = p.length_squared();

        // Ensure the vector is not too small (avoid near-zero vectors)
        // Length squared should be between a small value (e.g., 1e-6) and 1
        if length_sq > 0.000001 && length_sq <= 1.0 {
            return p / length_sq.sqrt(); // Normalize the vector
        }
    }
}

pub fn is_near_zero_length(vec: Vec3, epsilon: f32) -> bool {
    vec.length_squared() < epsilon * epsilon
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_cosine_weighted_hemisphere(normal: &Vec3) -> Vec3 {
    let r1: f32 = rand::random::<f32>();
    let r2: f32 = rand::random::<f32>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    let tangent = normal.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    let bitangent = tangent.cross(*normal);

    x * tangent + y * bitangent + z * *normal
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

pub fn clamp_color(color: &mut Vec3) {
    color.x = color.x.clamp(0.0, 1.0);
    color.y = color.y.clamp(0.0, 1.0);
    color.z = color.z.clamp(0.0, 1.0);
}
