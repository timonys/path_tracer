use crate::path_tracer::path_trace_components::AccumulatedSampleBufferComponent;
use crate::renderer::render_components::FramebufferComponent;
use crate::renderer::render_system::render;
use crate::window::window_component::WindowComponent;
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

        world.get::<&WindowComponent>(|window| {
            let (width, height) = window.handle.get_size();
            world.set(FramebufferComponent::new(width, height));
        });

        system!(
            "render_to_framebuffer",
            world,
            &mut FramebufferComponent,
            &AccumulatedSampleBufferComponent
        )
        .singleton()
        .each(|(framebuffer, sample_buffer)| {
            render(framebuffer, sample_buffer);
        });
    }
}
