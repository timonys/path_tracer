use clap::Parser;
use flecs_ecs::prelude::*;

#[derive(Parser, Component, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PathTracerConfig {
    #[arg(short = 'w', long, default_value_t = 600)]
    pub width: usize,
    #[arg(short = 'y', long, default_value_t = 400)]
    pub height: usize,
    #[arg(short = 's', long, default_value_t = 150)]
    pub samples: i32,
    #[arg(short = 'd', long, default_value_t = 100)]
    pub max_depth: i32,
}
