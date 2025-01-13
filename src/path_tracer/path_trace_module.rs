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

        system!(
            "path_trace",
            world,
            &mut PathTracerComponent,
            &mut AccumulatedSampleBufferComponent
        )
        .each_entity(|entity, (path_tracer, sample_buffer)| {
            if !path_tracer.has_rendered {
                path_trace(entity, path_tracer, sample_buffer);
            }
        });
    }
}
