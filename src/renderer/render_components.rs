use flecs_ecs::prelude::*;

#[derive(Component)]
pub struct FramebufferComponent {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl FramebufferComponent {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }
}
