use clap::Parser;

use super::{DEFAULT_OUTPUT, DEFAULT_REFLECT_DEPTH, DEFAULT_SAMPLES, DEFAULT_WIDTH};

pub fn get_args() -> CliOpts {
    CliOpts::parse()
}

#[derive(Parser, Debug)]
#[clap(version = clap::crate_version!(), author = "Scott S. <scottschroeder@sent.com>")]
pub struct CliOpts {
    #[clap(short, long, global = true, parse(from_occurrences))]
    pub verbose: u8,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Test(Test),
    /// Render an image
    #[clap(subcommand)]
    Render(Render),
}

#[derive(Parser, Debug)]
pub enum Render {
    Random(RenderRandom),
    Demo(RenderDemo),
    Perlin(RenderPerlin),
    Earth(RenderEarth),
    BoxLight(RenderBoxLight),
    Cornell(RenderCornellBox),
}

#[derive(Parser, Debug)]
pub struct RenderCornellBox {
    #[clap(flatten)]
    pub config: RenderSettings,
}
#[derive(Parser, Debug)]
pub struct RenderBoxLight {
    #[clap(flatten)]
    pub config: RenderSettings,
}
#[derive(Parser, Debug)]
pub struct RenderRandom {
    #[clap(flatten)]
    pub config: RenderSettings,
    /// Render at night time!
    #[clap(long)]
    pub night: bool,
}
#[derive(Parser, Debug)]
pub struct RenderDemo {
    #[clap(flatten)]
    pub config: RenderSettings,
}

#[derive(Parser, Debug)]
pub struct RenderEarth {
    #[clap(flatten)]
    pub config: RenderSettings,
}

#[derive(Parser, Debug)]
pub struct RenderPerlin {
    #[clap(flatten)]
    pub config: RenderSettings,
}

#[derive(Parser, Debug)]
pub struct RenderSettings {
    /// Output file for image
    #[clap(short, long, default_value=DEFAULT_OUTPUT)]
    pub output: String,

    /// Set width of image in pixels
    #[clap(short, long, default_value=DEFAULT_WIDTH)]
    pub width: usize,

    /// Number of iterations to sample each pixel
    #[clap(short, long, default_value=DEFAULT_SAMPLES)]
    pub samples: usize,

    /// Maximum number of bounces
    #[clap(short, long, default_value=DEFAULT_REFLECT_DEPTH)]
    pub max_reflect: usize,

    /// Render on a single core
    #[clap(long)]
    pub single_threaded: bool,
}

#[derive(Parser, Debug)]
pub struct Test {}
