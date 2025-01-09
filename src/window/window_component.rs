use flecs_ecs::prelude::*;
use minifb::Window;

#[derive(Component)]
pub struct WindowComponent {
    pub width: usize,
    pub height: usize,
    pub title: String,
    pub handle: Option<Window>,
}

impl WindowComponent {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        Self {
            width,
            height,
            title: title.to_string(),
            handle: None,
        }
    }
}
