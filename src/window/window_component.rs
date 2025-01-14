use flecs_ecs::prelude::*;
use minifb::Window;

#[derive(Component)]
pub struct WindowComponent {
    pub handle: Window,
}

impl WindowComponent {
    pub fn new(window: Window) -> Self {
        WindowComponent { handle: window }
    }
}
