mod renderer;
mod window;

pub use crate::renderer::render_module::*;
pub use crate::renderer::render_system::*;
pub use crate::window::window_module::*;
use flecs_ecs::prelude::*;
fn main() {
    let width = 1200;
    let height = 800;

    let world = World::new();

    world.import::<WindowModule>();
    world.import::<RenderModule>();
    let _path_tracer = world
        .entity_named("window")
        .set(WindowComponent::new(width, height, "Timo's Path Tracer"))
        .set(FramebufferComponent::new(width, height));

    loop {
        world.progress();
    }
}
