use anyhow::Result;

pub mod util {
    mod color;
    mod vec3;
    pub use color::Color;
    pub use vec3::{Point, Ray, Vec3};
}
pub mod camera;
pub mod objects {
    mod hittable;
    pub mod scene;
    pub mod sphere;
    pub use hittable::Geometry;
}
pub mod image;

use objects::{scene::Scene, Geometry};
use util::{Color, Ray, Vec3};

const DEFAULT_WIDTH: &str = "640";
const DEFAULT_SAMPLES: &str = "100";
const DEFAULT_OUTPUT: &str = "out.png";

fn main() -> Result<()> {
    color_backtrace::install();
    let args = cli::get_args();
    cli::setup_logger(args.occurrences_of("verbosity"));
    log::trace!("Args: {:?}", args);

    match args.subcommand() {
        ("render", Some(sub_m)) => render_image(sub_m),
        ("", _) => Err(anyhow::anyhow!(
            "Please provide a command:\n{}",
            args.usage()
        )),
        subc => Err(anyhow::anyhow!(
            "Unknown command: {:?}\n{}",
            subc,
            args.usage()
        )),
    }
    .map_err(|e| {
        log::error!("{:?}", e);
        anyhow::anyhow!("unrecoverable {} failure", clap::crate_name!())
    })
}

fn skybox(r: &Ray) -> Color {
    let unit = r.direction.unit();
    let t = 0.5f64 * (unit.y() + 1f64);
    Color(Vec3::new(1.0, 1.0, 1.0).scale(1f64 - t) + Vec3::new(0.5, 0.7, 1.0).scale(t))
}

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    if let Some(r) = scene.hit(ray, 0.0, std::f64::INFINITY) {
        Color((Vec3::new(1.0, 1.0, 1.0) + r.normal).scale(0.5))
    } else {
        skybox(ray)
    }
}

fn render_image(args: &clap::ArgMatches) -> Result<()> {
    use rand::prelude::*;
    use rayon::prelude::*;

    let width = args.value_of("width").unwrap().parse::<usize>()?;
    let output = args.value_of("output").unwrap();
    let mut samples = args.value_of("samples").unwrap().parse::<usize>()?;
    if samples == 0 {
        log::warn!("samples set to 0, using 1");
        samples = 1;
    }

    let mut camera = camera::CameraBuilder::default();
    camera.width(width).aspect_ratio((16, 9));
    let camera = camera.build()?;

    let mut image = image::Image::from_dimm(camera.dimm);
    image.samples = samples;
    let scene = create_scene();
    log::trace!("render");
    image
        .scanlines_mut()
        .into_par_iter()
        .enumerate()
        .for_each_init(
            || rand::thread_rng(),
            |rng, (j, line)| {
                for i in 0..camera.dimm.width {
                    let mut c = Color::default();
                    for _ in 0..samples {
                        let r = camera
                            .pixel_ray(i as f64 + rng.gen::<f64>(), j as f64 + rng.gen::<f64>());
                        c += ray_color(&r, &scene);
                    }
                    line[i] = c
                }
            },
        );

    image::to_image(&image, output);
    Ok(())
}

fn create_scene() -> Scene {
    let mut scene = Scene::default();
    scene.add(objects::sphere::Sphere {
        center: util::Point(Vec3::new(0.0, -100.5, -1.0)),
        radius: 100.0,
    });
    scene.add(objects::sphere::Sphere {
        center: util::Point(Vec3::new(0.0, 0.0, -1.0)),
        radius: 0.5,
    });
    // scene.add(objects::sphere::Sphere {
    //     center: util::Point(Vec3::new(-1.0, 0.0, -1.0)),
    //     radius: 0.3,
    // });
    // scene.add(objects::sphere::Sphere {
    //     center: util::Point(Vec3::new(1.0, 0.0, -1.0)),
    //     radius: 0.3,
    // });
    scene
}

mod cli {
    use clap::SubCommand;

    use crate::{DEFAULT_OUTPUT, DEFAULT_SAMPLES, DEFAULT_WIDTH};

    pub fn setup_logger(level: u64) {
        let mut builder = pretty_env_logger::formatted_timed_builder();

        let noisy_modules: &[&str] = &[];

        let log_level = match level {
            //0 => log::Level::Error,
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        };

        if level > 1 && level < 4 {
            for module in noisy_modules {
                builder.filter_module(module, log::LevelFilter::Info);
            }
        }

        builder.filter_level(log_level);
        builder.format_timestamp_millis();
        builder.init();
    }

    pub fn get_args() -> clap::ArgMatches<'static> {
        clap::App::new(clap::crate_name!())
            .version(clap::crate_version!())
            .about(clap::crate_description!())
            .setting(clap::AppSettings::DeriveDisplayOrder)
            .arg(
                clap::Arg::with_name("verbosity")
                    .short("v")
                    .multiple(true)
                    .global(true)
                    .help("Sets the level of verbosity"),
            )
            .subcommand(
                SubCommand::with_name("render")
                    .arg(
                        clap::Arg::with_name("width")
                            .short("w")
                            .takes_value(true)
                            .default_value(DEFAULT_WIDTH)
                            .help("sets the pixel width"),
                    )
                    .arg(
                        clap::Arg::with_name("samples")
                            .takes_value(true)
                            .short("s")
                            .default_value(DEFAULT_SAMPLES)
                            .help("number of samples per pixel"),
                    )
                    .arg(
                        clap::Arg::with_name("output")
                            .short("o")
                            .default_value(DEFAULT_OUTPUT)
                            .takes_value(true)
                            .help("path for output image"),
                    ),
            )
            .get_matches()
    }
}
