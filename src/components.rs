use flecs_ecs::prelude::Component;
use glam::Vec3;

#[derive(Debug, Component)]
enum ShapeComponent {
    Sphere { center: Vec3, radius: f32 },
}

#[derive(Debug, Component)]
struct MaterialComponent {
    diffuse_color: Vec3,
}
#[derive(Debug, Component)]
pub struct PathTracerComponent {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub accumulated_colors: Vec<Vec3>, //Colors for each pixel (RGB)
}

impl PathTracerComponent {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            samples_per_pixel: 1,
            accumulated_colors: vec![Vec3::new(0.0, 0.0, 0.0); width * height],
        }
    }
}

#[derive(Debug, Component)]
pub struct RenderDataComponent {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // RGBA buffer for display
}

impl RenderDataComponent {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
}

#[derive(Debug, Component)]
pub struct CameraComponent {
    pub position: Vec3,
    pub direction: Vec3,
    pub fov: f32,
}
