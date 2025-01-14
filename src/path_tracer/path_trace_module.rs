use crate::config::PathTracerConfig;
use crate::path_tracer::path_trace_components::*;
use crate::path_tracer::path_tracer_systems::*;
use flecs_ecs::prelude::*;

#[derive(Component)]
pub struct PathTracerComponentModule;
#[derive(Component)]
pub struct PathTracerModule;

impl Module for PathTracerComponentModule {
    fn module(world: &World) {
        world.module::<PathTracerComponentModule>("path_tracer::components");
        world.component_named::<PathTracerComponent>("PathTracerComponent");
        world.component_named::<ShapeComponent>("ShapeComponent");
        world.component_named::<MaterialComponent>("MaterialComponent");
        world.component_named::<AccumulatedSampleBufferComponent>(
            "AccumulatedSampleBufferComponent",
        );
    }
}

impl Module for PathTracerModule {
    fn module(world: &World) {
        world.import::<PathTracerComponentModule>();
        world.module::<PathTracerModule>("path_tracer::systems");

        world.get::<&PathTracerConfig>(|config| {
            world.set(AccumulatedSampleBufferComponent::new(
                config.width,
                config.height,
            ));

            world.set(PathTracerComponent::new(config.width as i32, config.height as i32,config.max_depth, config.samples));
            world.set(RayBufferComponent::new());

            system!("generate_rays", world, &mut CameraComponent, &mut RayBufferComponent($), &PathTracerComponent($))
            .kind::<flecs::pipeline::OnStart>()
            .each(|(cam, ray_buffer, path_tracer)| {
                generate_rays( cam, path_tracer, ray_buffer);
            });

            system!("trace_color", world, &mut RayBufferComponent, &mut AccumulatedSampleBufferComponent($), &PathTracerComponent($))
            .kind::<flecs::pipeline::OnStart>()
            .each_entity(|entity,(ray_buffer, sample_buffer, path_tracer)| {
                trace_color(entity, path_tracer, ray_buffer, sample_buffer);
            });
        });
    }
}
