use crate::path_tracer::materials::*;
use crate::path_tracer::shapes::*;
use flecs_ecs::prelude::*;
use glam::Vec3;
use std::sync::Arc;

#[derive(Component)]
pub struct PathTracerComponent {
    pub max_depth: i32,
    pub sample_amount: i32,
    pub has_rendered: bool,
}

impl PathTracerComponent {
    pub fn new(max_depth: i32, sample_amount: i32) -> Self {
        Self {
            max_depth,
            sample_amount,
            has_rendered: false,
        }
    }
}

#[derive(Component)]
pub struct CameraComponent {
    pub pos: Vec3, // Camera position in world space
    pub viewport_width: usize,
    pub viewport_height: usize,
}

impl CameraComponent {
    pub fn new(pos: Vec3, viewport_width: usize, viewport_height: usize) -> Self {
        Self {
            pos,
            viewport_width,
            viewport_height,
        }
    }
}

#[derive(Component)]
pub struct AccumulatedSampleBufferComponent {
    pub width: usize,
    pub height: usize,
    pub sample_data: Vec<Vec3>,
}

impl AccumulatedSampleBufferComponent {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            sample_data: vec![Vec3::default(); width * height],
        }
    }
}

#[derive(Component)]
pub struct ShapeComponent {
    pub shape: Box<dyn Shape>,
}

#[derive(Component)]
pub struct MaterialComponent {
    pub material: Arc<dyn Material>,
}
