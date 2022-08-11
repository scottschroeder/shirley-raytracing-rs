use clap::Parser;

use super::DEFAULT_WIDTH;
use super::DEFAULT_OUTPUT;
use super::DEFAULT_SAMPLES;

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
    Render(Render),
}

#[derive(Parser, Debug)]
pub struct Render {
    /// Output file for image
    #[clap(short, long, default_value=DEFAULT_OUTPUT)]
    pub output: String,

    /// Set width of image in pixels
    #[clap(short, long, default_value=DEFAULT_WIDTH)]
    pub width: usize,

    /// Number of iterations to sample each pixel
    #[clap(short, long, default_value=DEFAULT_SAMPLES)]
    pub samples: usize,

    /// Generate a random scene
    #[clap(long)]
    pub random: bool,

    /// Render on a single core
    #[clap(long)]
    pub single_threaded: bool
}

#[derive(Parser, Debug)]
pub struct Test {}
