use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PathTracerConfig {
    #[arg(short = 'w', long, default_value_t = 600)]
    pub width: usize,
    #[arg(short = 'y', long, default_value_t = 400)]
    pub height: usize,
    #[arg(short = 's', long, default_value_t = 30)]
    pub samples: i32,
    #[arg(short = 'd', long, default_value_t = 10)]
    pub max_depth: i32,
}
