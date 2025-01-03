use glam::Vec3;

#[derive(Debug)]
enum ShapeComponent {
    Sphere { center: Vec3, radius: f32 },
}

#[derive(Debug)]
struct MaterialComponent {
    diffuse_color: [f32; 3],
}
#[derive(Debug)]
pub struct PathTracerComponent {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub accumulated_colors: Vec<f32>, //Colors for each pixel (RGB)
}

impl PathTracerComponent {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            samples_per_pixel: 1,
            accumulated_colors: vec![0.0; width * height * 3],
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
pub struct CameraComponent {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub fov: f32,
}
