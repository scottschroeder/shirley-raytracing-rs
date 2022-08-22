use clap::Parser;

const DEFAULT_WIDTH: &str = "640";
const DEFAULT_SAMPLES: &str = "100";
const DEFAULT_REFLECT_DEPTH: &str = "50";
const DEFAULT_OUTPUT: &str = "out.png";

const DEFAULT_CAMERA_VFOV: &str = "20.0";
const DEFAULT_CAMERA_FOCAL_LENGTH: &str = "1.0";
const DEFAULT_CAMERA_APERTURE: &str = "0.001";

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
    Saved(RenderSaved),
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
    #[clap(flatten)]
    pub camera: CameraSettings,
}
#[derive(Parser, Debug)]
pub struct RenderBoxLight {
    #[clap(flatten)]
    pub config: RenderSettings,
    #[clap(flatten)]
    pub camera: CameraSettings,
}
#[derive(Parser, Debug)]
pub struct RenderSaved {
    #[clap(flatten)]
    pub config: RenderSettings,

    #[clap(flatten)]
    pub camera: CameraSettings,

    /// Input file for scene_data
    pub scene_input: String,
}
#[derive(Parser, Debug)]
pub struct RenderRandom {
    #[clap(flatten)]
    pub config: RenderSettings,
    #[clap(flatten)]
    pub camera: CameraSettings,
    /// Render at night time!
    #[clap(long)]
    pub night: bool,
    /// Output file for scene_data
    #[clap(long)]
    pub scene_output: Option<String>,
}
#[derive(Parser, Debug)]
pub struct RenderDemo {
    #[clap(flatten)]
    pub config: RenderSettings,
    #[clap(flatten)]
    pub camera: CameraSettings,
}

#[derive(Parser, Debug)]
pub struct RenderEarth {
    #[clap(flatten)]
    pub config: RenderSettings,
    #[clap(flatten)]
    pub camera: CameraSettings,
}

#[derive(Parser, Debug)]
pub struct RenderPerlin {
    #[clap(flatten)]
    pub config: RenderSettings,
    #[clap(flatten)]
    pub camera: CameraSettings,
}

#[derive(Parser, Debug)]
pub struct RenderSettings {
    /// Output file for image
    #[clap(short, long, default_value=DEFAULT_OUTPUT)]
    pub output: String,

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
pub struct CameraSettings {
    /// Set width of image in pixels
    #[clap(short, long, default_value=DEFAULT_WIDTH)]
    pub width: usize,

    /// Camera Field of View
    #[clap(long, default_value=DEFAULT_CAMERA_VFOV)]
    pub camera_fov: f64,

    /// Camera Focal Length
    #[clap(long, default_value=DEFAULT_CAMERA_FOCAL_LENGTH)]
    pub camera_focal_length: f64,
    ///
    /// Camera Focal Length
    #[clap(long, default_value=DEFAULT_CAMERA_APERTURE)]
    pub camera_aperture: f64,

    /// Camera AspectRatio
    #[clap(long, value_enum, default_value_t=CameraAspectRatio::Std3x2)]
    pub camera_aspect_ratio: CameraAspectRatio,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum CameraAspectRatio {
    Std3x2,
    Std16x9,
    Std16x10,
    Square,
    TargetIphone,
}

impl CameraAspectRatio {
    pub fn ratio(&self) -> (u32, u32) {
        match self {
            CameraAspectRatio::Std3x2 => (3, 2),
            CameraAspectRatio::Std16x9 => (16, 9),
            CameraAspectRatio::Std16x10 => (16, 10),
            CameraAspectRatio::Square => (1, 1),
            CameraAspectRatio::TargetIphone => (1170, 2532),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Test {}
