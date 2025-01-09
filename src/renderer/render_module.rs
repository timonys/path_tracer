use super::render_components::FramebufferComponent;
use crate::renderer::render_system::*;
use flecs_ecs::prelude::*;

#[derive(Component)]
pub struct RenderComponentModule;
#[derive(Component)]
pub struct RenderModule;

impl Module for RenderComponentModule {
    fn module(world: &World) {
        world.module::<RenderComponentModule>("renderer::components");
        world.component_named::<FramebufferComponent>("FramebufferComponent");
    }
}

impl Module for RenderModule {
    fn module(world: &World) {
        world.import::<RenderComponentModule>();
        world.module::<RenderModule>("renderer::systems");

        world
            .system_named::<&mut FramebufferComponent>("render_system")
            .each(|framebuffer| {
                render(framebuffer);
            });
    }
}
