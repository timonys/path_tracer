use crate::path_tracer::materials::*;
use crate::path_tracer::path_tracer_structs::Ray;
use crate::path_tracer::shapes::*;
use flecs_ecs::prelude::*;
use glam::Vec3;

#[derive(Component)]
pub struct PathTracerComponent {
    pub width: i32,
    pub height: i32,
    pub max_depth: i32,
    pub sample_amount: i32,
}

impl PathTracerComponent {
    pub fn new(width: i32, height: i32, max_depth: i32, sample_amount: i32) -> Self {
        Self {
            width,
            height,
            max_depth,
            sample_amount,
        }
    }
}

#[derive(Component)]
pub struct CameraComponent {
    pub pos: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_upper_left_pos: Vec3,
}

impl CameraComponent {
    pub fn new(
        pos: Vec3,
        viewport_width: usize,
        viewport_height: usize,
        cam_height: f32,
        focal_length: f32,
    ) -> Self {
        let cam_width = cam_height * (viewport_width as f32 / viewport_height as f32);

        // Now you can safely calculate the vectors
        let viewport_u = Vec3::new(cam_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -cam_height, 0.0);

        let pixel_delta_u = viewport_u / viewport_width as f32;
        let pixel_delta_v = viewport_v / viewport_height as f32;

        let viewport_upper_left =
            pos - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel_upper_left_pos = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            pos,
            pixel_delta_u,
            pixel_delta_v,
            pixel_upper_left_pos,
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
pub struct RayBufferComponent {
    pub rays: Vec<Ray>,
    pub indices: Vec<(i32, i32)>,
}

impl RayBufferComponent {
    // Constructor to initialize the struct with default values
    pub fn new() -> Self {
        RayBufferComponent {
            rays: Vec::new(),
            indices: Vec::new(),
        }
    }
}

#[derive(Component)]
pub struct ShapeComponent {
    pub shape: Box<dyn Shape>,
}

#[derive(Component)]
pub struct MaterialComponent {
    pub material: MaterialType,
}

#[derive(Component)]
pub struct TransformComponent {
    pub pos: Vec3,
}

impl TransformComponent {
    pub fn new(pos: Vec3) -> Self {
        TransformComponent { pos }
    }
}
