use crate::path_tracer::path_trace_components::*;
use crate::renderer::render_components::FramebufferComponent;
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

        system!("render_to_framebuffer", world, &mut FramebufferComponent).each_entity(
            |e, framebuffer| {
                render(e, framebuffer);
            },
        );
    }
}
