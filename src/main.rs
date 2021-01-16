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
    pub mod sphere;
    pub use hittable::Hittable;
}
pub mod image;

use util::{Color, Ray, Vec3};

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

fn ray_color(ray: &Ray) -> Color {
    let s = util::Point(Vec3::new(0.0, 0.0, -1.0));
    if let Some(t) = objects::sphere::hit_sphere(s, 0.5, ray) {
        let norm = (ray.at(t).0 - Vec3::new(0.0, 0.0, -1.0)).unit();
        Color((Vec3::new(1.0, 1.0, 1.0) + norm).scale(0.5))
    } else {
        skybox(ray)
    }
}

fn render_image(args: &clap::ArgMatches) -> Result<()> {
    use rayon::prelude::*;

    let width = if let Some(w) = args.value_of("width") {
        w.parse::<usize>()?
    } else {
        256
    };

    let output = args.value_of("output").unwrap_or("out.png");

    let mut camera = camera::CameraBuilder::default();
    camera.width(width).aspect_ratio((16, 9));
    let camera = camera.build()?;

    let mut image = image::Image::from_dimm(camera.dimm);

    log::trace!("render");
    image
        .scanlines_mut()
        .into_par_iter()
        .enumerate()
        .for_each(|(j, line)| {
            for i in 0..camera.dimm.width {
                let r = camera.pixel_ray(i, j);
                let c = ray_color(&r);
                line[i] = c
            }
        });

    image::to_image(&image, output);
    Ok(())
}

mod cli {
    use clap::SubCommand;

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
                            .help("sets the pixel width"),
                    )
                    .arg(
                        clap::Arg::with_name("output")
                            .short("o")
                            .takes_value(true)
                            .help("path for output image"),
                    ),
            )
            .get_matches()
    }
}
