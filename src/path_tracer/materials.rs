use crate::math::*;
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

#[derive(Clone)]
pub enum MaterialType {
    Lambert(Lambert),
    Metal(Metal),
    Glossy(Glossy),
    Dielectric(Dielectric),
}

impl Material for MaterialType {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        match self {
            MaterialType::Lambert(material) => {
                material.scatter(ray, hit, attenuation, scattered_ray)
            }
            MaterialType::Metal(material) => material.scatter(ray, hit, attenuation, scattered_ray),
            MaterialType::Glossy(material) => {
                material.scatter(ray, hit, attenuation, scattered_ray)
            }
            MaterialType::Dielectric(material) => {
                material.scatter(ray, hit, attenuation, scattered_ray)
            }
        }
    }
}

#[derive(Clone)]
pub struct Lambert {
    pub albedo: Vec3,
}

impl Lambert {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

#[allow(unused_variables)]
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

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f32) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray.dir, hit.normal);
        let scatter_dir = reflected + random_unit_vector() * self.fuzziness;

        create_scatter_data(scattered_ray, attenuation, hit, scatter_dir, self.albedo);
        true
    }
}

#[derive(Clone)]
pub struct Glossy {
    pub albedo: Vec3,
    pub reflectivity: f32,
}

impl Glossy {
    pub fn new(albedo: Vec3, reflectivity: f32) -> Self {
        Self {
            albedo,
            reflectivity,
        }
    }
}

impl Material for Glossy {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray.dir, hit.normal);
        let scatter_dir = if rand::random::<f32>() < self.reflectivity {
            reflected
        } else {
            hit.normal + random_unit_vector()
        };

        create_scatter_data(scattered_ray, attenuation, hit, scatter_dir, self.albedo);
        scattered_ray.dir.dot(hit.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let refraction_ratio = if hit.front_facing {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = f32::min((-ray.dir).dot(hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let scatter_dir = if cannot_refract
            || schlick_reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            reflect(ray.dir, hit.normal)
        } else {
            refract(ray.dir, hit.normal, refraction_ratio)
        };

        create_scatter_data(scattered_ray, attenuation, hit, scatter_dir, Vec3::ONE);
        true
    }
}

fn create_scatter_data(
    scattered_ray: &mut Ray,
    attenuation: &mut Vec3,
    hit: &Hit,
    scatter_dir: Vec3,
    albedo: Vec3,
) {
    *attenuation = albedo;
    *scattered_ray = Ray::new(hit.intersection, scatter_dir);
}
