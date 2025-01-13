use super::materials::Material;
use glam::Vec3;
use std::sync::Arc;

pub struct Hit {
    pub intersection: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_facing: bool,
    pub material: Arc<dyn Material>,
}

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, intersection_point: f32) -> Vec3 {
        self.origin + intersection_point * self.dir
    }
}

pub struct AccumulatedSampleData {
    pub color: Vec3,
}
