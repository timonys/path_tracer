use crate::config::PathTracerConfig;
use crate::path_tracer::materials::{Dielectric, Glossy, Lambert, MaterialType, Metal};
use crate::path_tracer::path_trace_components::CameraComponent;
use crate::path_tracer::path_trace_components::*;
use crate::path_tracer::path_trace_module::PathTracerModule;
use crate::path_tracer::shapes::{Plane, Sphere, Triangle};
use crate::renderer::render_module::RenderModule;
use crate::window::window_component::WindowComponent;
use crate::window::window_module::WindowModule;
use flecs_ecs::prelude::*;
use glam::Vec3;

use clap::Parser;

pub struct PathTracerApp {
    world: World,
}

impl PathTracerApp {
    pub fn new() -> Self {
        PathTracerApp {
            world: World::new(),
        }
    }

    pub fn start(&self) {
        self.world.set(PathTracerConfig::parse());

        let mut width: usize = 0;
        let mut height: usize = 0;
        self.world.get::<&PathTracerConfig>(|config| {
            width = config.width;
            height = config.height;
        });

        self.world.import::<PathTracerModule>();
        self.world.import::<WindowModule>();
        self.world.import::<RenderModule>();

        //shapes
        let _sphere = Sphere::new(0.5);
        let _sphere2 = Sphere::new(0.3);
        let _plane = Plane::new(Vec3::new(0.0, 1.0, 0.0));
        let _triangle = Triangle::new(
            Vec3::new(-0.4, 0.6, 0.0),
            Vec3::new(-1.5, -3.0, 0.0),
            Vec3::new(1.0, -1.0, -0.5),
        );

        //materials
        let _lambert_mat = MaterialType::Lambert(Lambert::new(Vec3::new(0.1, 0.2, 0.5)));
        let _lambert_mat_gloss = MaterialType::Glossy(Glossy::new(Vec3::new(0.9, 0.9, 0.9), 0.6));
        let _metal = MaterialType::Metal(Metal::new(Vec3::new(0.7, 0.7, 0.7), 0.5));
        let _transparent_mat = MaterialType::Dielectric(Dielectric::new(1.04));

        //entities
        let _camera = self.world.entity_named("camera").set(CameraComponent::new(
            Vec3::new(0.0, 0.0, 0.0),
            width,
            height,
            2.0,
            1.0,
        ));

        let _plane_entity = self
            .world
            .entity()
            .set(ShapeComponent {
                shape: Box::new(_plane),
            })
            .set(MaterialComponent {
                material: _lambert_mat,
            })
            .set(TransformComponent::new(Vec3::new(0.0, -0.4, 0.0)));

        let _triangle_entity = self
            .world
            .entity()
            .set(ShapeComponent {
                shape: Box::new(_triangle),
            })
            .set(MaterialComponent { material: _metal })
            .set(TransformComponent::new(Vec3::new(-0.5, 0.3, -1.0)));

        let _sphere_entity = self
            .world
            .entity()
            .set(ShapeComponent {
                shape: Box::new(_sphere),
            })
            .set(MaterialComponent {
                material: _lambert_mat_gloss,
            })
            .set(TransformComponent::new(Vec3::new(0.0, 0.1, -1.5)));

        let _sphere_entity2 = self
            .world
            .entity()
            .set(ShapeComponent {
                shape: Box::new(_sphere2),
            })
            .set(MaterialComponent {
                material: _transparent_mat,
            })
            .set(TransformComponent::new(Vec3::new(0.8, 0.3, -1.0)));
    }
}

impl PathTracerApp {
    pub fn run(&mut self) {
        let mut should_run = true;
        while should_run {
            self.world.progress();

            self.world
                .get::<&WindowComponent>(|window| should_run = window.handle.is_open());
        }
    }
}
