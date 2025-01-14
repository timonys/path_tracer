use super::materials::MaterialType;
use crate::path_tracer::path_tracer_structs::*;
use glam::Vec3;

pub trait Shape {
    fn intersect(
        &self,
        pos: &Vec3,
        ray: &Ray,
        material: &MaterialType,
        ray_min: f32,
        ray_max: f32,
    ) -> Option<Hit>;
}

pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}

impl Shape for Sphere {
    fn intersect(
        &self,
        pos: &Vec3,
        ray: &Ray,
        material: &MaterialType,
        ray_min: f32,
        ray_max: f32,
    ) -> Option<Hit> {
        let oc = *pos - ray.origin;

        let a = ray.dir.length_squared();
        let b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (b - sqrt_discriminant) / a;

        if t <= ray_min || ray_max <= t {
            t = (b + sqrt_discriminant) / a;
            if t <= ray_min || ray_max <= t {
                return None;
            }
        }

        let intersection = ray.at(t);
        let mut normal = (intersection - *pos) / self.radius;

        let front_facing = ray.dir.dot(normal) > 0.0;
        if front_facing {
            normal = -normal;
        }

        let hit = Hit {
            intersection,
            normal,
            t,
            front_facing,
            material: material.clone(),
        };
        Some(hit)
    }
}

pub struct Triangle {
    pub p1_offset_center: Vec3,
    pub p2_offset_center: Vec3,
    pub p3_offset_center: Vec3,
}

impl Triangle {
    pub fn new(p1_offset_center: Vec3, p2_offset_center: Vec3, p3_offset_center: Vec3) -> Self {
        Triangle {
            p1_offset_center,
            p2_offset_center,
            p3_offset_center,
        }
    }
}

impl Shape for Triangle {
    fn intersect(
        &self,
        pos: &Vec3,
        ray: &Ray,
        material: &MaterialType,
        ray_min: f32,
        ray_max: f32,
    ) -> Option<Hit> {
        let point1 = *pos + self.p1_offset_center;
        let point2 = *pos + self.p2_offset_center;
        let point3 = *pos + self.p3_offset_center;

        let edge1 = point2 - point1;
        let edge2 = point3 - point1;

        // Calculate the cross product of ray direction and edge2
        let h = ray.dir.cross(edge2);
        let a = edge1.dot(h);

        // If a is close to zero, the ray is parallel to the triangle's plane
        if a.abs() < f32::EPSILON {
            return None;
        }

        // Calculate the inverse of a
        let f = 1.0 / a;
        let s = ray.origin - point1;
        let u = f * s.dot(h);

        // Check if intersection lies within the triangle
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.dir.dot(q);

        // If intersection is outside the triangle, return None
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Calculate the distance along the ray to the intersection
        let t = f * edge2.dot(q);

        // Ensure the intersection is within the ray's bounds
        if t <= ray_min || t >= ray_max {
            return None;
        }

        // Compute the intersection point and the normal of the triangle
        let intersection = ray.origin + ray.dir * t;
        let normal = edge1.cross(edge2).normalize(); // Calculate and normalize the normal

        // Check if the ray hits the front face of the triangle
        let front_facing = ray.dir.dot(normal) < 0.0;

        // Return the Hit object with the computed values
        Some(Hit {
            intersection,
            normal,
            t, // Distance from ray origin
            front_facing,
            material: material.clone(),
        })
    }
}

pub struct Plane {
    pub normal: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3) -> Self {
        Plane { normal }
    }
}

impl Shape for Plane {
    fn intersect(
        &self,
        pos: &Vec3,
        ray: &Ray,
        material: &MaterialType,
        ray_min: f32,
        ray_max: f32,
    ) -> Option<Hit> {
        let denom = ray.dir.dot(self.normal);

        // If the ray is parallel to the plane, there is no intersection
        if denom.abs() < f32::EPSILON {
            return None;
        }

        // Calculate the intersection distance along the ray
        let t = (*pos - ray.origin).dot(self.normal) / denom;

        // If the intersection is behind the ray origin or outside of ray bounds, no hit
        if t < ray_min || t > ray_max {
            return None;
        }

        // Calculate the intersection point
        let intersection = ray.origin + ray.dir * t;

        // The normal of the plane is constant at every point on the plane
        let normal = self.normal;

        // Check if the ray is hitting the front face of the plane
        let front_facing = ray.dir.dot(normal) < 0.0;

        // Return the Hit object with the computed values
        Some(Hit {
            intersection,
            normal,
            t, // Distance from ray origin
            front_facing,
            material: material.clone(),
        })
    }
}
