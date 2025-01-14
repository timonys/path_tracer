use super::materials::MaterialType;
use glam::Vec3;

pub struct Hit {
    pub intersection: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_facing: bool,
    pub material: MaterialType,
}

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }
    pub fn at(&self, intersection_point: f32) -> Vec3 {
        self.origin + intersection_point * self.dir
    }
}
