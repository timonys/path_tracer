mod path_tracer {
    pub mod renderer;
    pub mod window;
}

mod ecs {
    pub mod components;
}

use ecs::components::CameraComponent;
use ecs::components::PathTracerComponent;
use ecs::components::RenderDataComponent;
//use ecs::systems::path_trace_system;
//use flecs_ecs::*;
use path_tracer::renderer::Renderer;

use path_tracer::window::WindowWrapper; // Use WindowWrapper from the rendering module
fn main() {
    let width = 1200;
    let height = 800;
    println!("Hello world");
    /* let mut world: World::new();
    world.register::<CameraComponent>();
    world.register::<PathTracerComponent>();
    world.register::<RenderDataComponent>();
    //Create ECS entities
    let camera = world.spawn(CameraComponent {
        position: [0.0, 0.0, -5.0],
        direction: [0.0, 0.0, 1.0],
        fov: 60.0,
    });

    let path_tracer_entity = world.spawn(PathTracerComponent::new(width, height));
    let render_data_entity = world.spawn(RenderDataComponent::new(width, height));
    */
    let renderer = Renderer::new(width, height);
    let mut window = WindowWrapper::new("Timo's Path Tracer", width, height);

    while window.is_open() {
        // ECS updates
        // path_trace_system(&mut world);
        //tonemap_system(&mut world);

        // Fetch render data from ECS
      //  let render_data = world
         //   .query::<&RenderDataComponent>()
          //  .iter()
        //    .next()
         //   .expect("RenderDataComponent missing")
          //  .1;

        // Render the buffer
        //renderer.render(&render_data.buffer, &mut window);
    }
    
}
