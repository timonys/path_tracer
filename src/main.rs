mod config;
mod path_tracer;
mod renderer;
mod utils;
mod window;

use clap::Parser;
use config::PathTracerConfig;
use flecs_ecs::prelude::*;
use glam::Vec3;
use minifb::{Window, WindowOptions};
use path_tracer::materials::Lambert;
use path_tracer::path_trace_components::*;
use path_tracer::path_trace_module::PathTracerModule;
use path_tracer::shapes::Plane;
use path_tracer::shapes::Sphere;
use path_tracer::shapes::Triangle;
use renderer::render_module::*;
use renderer::render_system::*;
use std::sync::Arc;
use window::window_module::*;

fn main() {
    let path_tracer_config = PathTracerConfig::parse();
    let width = path_tracer_config.width;
    let height = path_tracer_config.height;
    let samples = path_tracer_config.samples;
    let max_depth = path_tracer_config.max_depth;
    let world = World::new();

    world.import::<PathTracerModule>();
    world.import::<RenderModule>();
    world.import::<WindowModule>();

    let minifb_window = Window::new(
        "Timo's Path Tracer",
        width,
        height,
        WindowOptions::default(),
    )
    .expect("Failed to build window");

    let _renderer = world
        .entity_named("renderer")
        .set(WindowComponent::new(minifb_window))
        .set(FramebufferComponent::new(width, height));

    let _path_tracer = world
        .entity_named("path_tracer")
        .set(AccumulatedSampleBufferComponent::new(width, height))
        .set(PathTracerComponent::new(max_depth, samples));

    let _camera = world.entity_named("camera").set(CameraComponent::new(
        Vec3::new(0.0, 0.0, 0.0),
        width,
        height,
    ));

    let _sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.5), 0.5);
    let _sphere2 = Sphere::new(Vec3::new(0.2, 0.3, -1.0), 0.3);

    let _lambert_mat = Lambert::new(Vec3::new(0.1, 0.2, 0.5));
    let _lambert_mat_2 = Lambert::new(Vec3::new(0.3, 0.3, 0.5));
    let _lambert_mat_plane = Lambert::new(Vec3::new(0.1, 0.1, 0.2));

    let _plane = Plane::new(Vec3::new(0.0, -0.4, 0.0), Vec3::new(0.0, 1.0, 0.0));

    let _sphere_entity = world
        .entity()
        .set(ShapeComponent {
            shape: Box::new(_sphere),
        })
        .set(MaterialComponent {
            material: Arc::new(_lambert_mat),
        });

    let _sphere_entity2 = world
        .entity()
        .set(ShapeComponent {
            shape: Box::new(_sphere2),
        })
        .set(MaterialComponent {
            material: Arc::new(_lambert_mat_2),
        });

    let _triangle = Triangle {
        p1: Vec3::new(0.0, 0.0, -1.0),
        p2: Vec3::new(0.2, 0.5, -1.0),
        p3: Vec3::new(0.8, -0.9, -1.0),
    };

    let _plane_entity = world
        .entity()
        .set(ShapeComponent {
            shape: Box::new(_plane),
        })
        .set(MaterialComponent {
            material: Arc::new(_lambert_mat_plane),
        });
    loop {
        world.progress();
    }
}
