use crate::utils::rand_vec3_between;
use glam::Vec3;

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

#[allow(dead_code)]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[allow(dead_code)]
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

pub fn reflect(in_vec: Vec3, n: Vec3) -> Vec3 {
    in_vec - 2.0 * in_vec.dot(n) * n
}

pub fn refract(in_vec: Vec3, normal: Vec3, refraction_ratio: f32) -> Vec3 {
    let cos_theta = f32::min((-in_vec).dot(normal), 1.0);
    let r_out_perp = refraction_ratio * (in_vec + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}