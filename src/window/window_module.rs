pub use crate::window::window_component::*;
pub use crate::window::window_system::*;
use crate::FramebufferComponent;
use flecs_ecs::prelude::*;

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

        system!("run_window", world, &mut WindowComponent, &mut FramebufferComponent).each(
            |(window, framebuffer)| {
                run_window(window, framebuffer);
            },
        );
    }
}
