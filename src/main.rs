mod background;
mod config;
mod math;
mod path_tracer;
mod path_tracer_app;
mod renderer;
mod utils;
mod window;

use path_tracer_app::PathTracerApp;
use renderer::render_system::*;
use window::window_module::*;

fn main() {
    let mut path_tracer_application = PathTracerApp::new();
    path_tracer_application.start();
    path_tracer_application.run();
}
