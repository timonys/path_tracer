use crate::config::PathTracerConfig;
pub use crate::window::window_component::WindowComponent;
pub use crate::window::window_system::update_window;
use crate::FramebufferComponent;
use flecs_ecs::prelude::*;
use minifb::{Window, WindowOptions};

#[derive(Component)]
pub struct WindowComponentModule;
#[derive(Component)]
pub struct WindowModule;

impl Module for WindowComponentModule {
    fn module(world: &World) {
        world.module::<WindowComponentModule>("window::components");
        world.component_named::<WindowComponent>("WindowComponent");
    }
}

impl Module for WindowModule {
    fn module(world: &World) {
        world.import::<WindowComponentModule>();
        world.module::<WindowModule>("window::systems");

        let mut width: usize = 0;
        let mut height: usize = 0;
        world.get::<&PathTracerConfig>(|config| {
            width = config.width;
            height = config.height;
        });

        let minifb_window = Window::new(
            "Timo's Path Tracer",
            width,
            height,
            WindowOptions::default(),
        )
        .expect("Failed to build window");

        world.set(WindowComponent::new(minifb_window));

        system!(
            "update_window",
            world,
            &mut WindowComponent,
            &FramebufferComponent
        )
        .singleton()
        .each(|(window, framebuffer)| {
            update_window(window, framebuffer);
        });
    }
}
