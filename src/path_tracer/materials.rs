use crate::path_tracer::path_tracer_structs::*;
use crate::utils::*;
use glam::Vec3;
pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool;
}

pub struct Lambert {
    pub albedo: Vec3,
}

impl Lambert {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambert {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_dir = (hit.normal + random_unit_vector()).normalize();

        if is_near_zero_length(scatter_dir, 1e-8) {
            scatter_dir = hit.normal;
        }

        *attenuation = self.albedo;
        *scattered_ray = Ray {
            origin: hit.intersection,
            dir: scatter_dir,
        };
        true
    }
}
