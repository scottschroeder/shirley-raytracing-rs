pub mod util {
    mod color;
    pub mod math;
    mod vec3;
    pub use color::Color;
    pub use vec3::{Point, Ray, Vec3};
}
pub mod camera;
pub mod objects {
    mod hittable;
    pub mod material;
    pub mod scene;
    pub mod sphere;
    pub use hittable::{Geometry, Hittable};
}
pub mod image;

use crate::objects::{
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};
use anyhow::Result;
use camera::{Camera, CameraPosition};
use objects::{scene::Scene, Hittable};
use rand::prelude::ThreadRng;
use util::{math::random_real, Color, Point, Ray, Vec3};

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

fn ray_color(incoming: &Ray, scene: &Scene, mut max_depth: usize) -> Color {
    let mut ray = *incoming;
    let mut attenuation = Color::ones();

    while max_depth > 0 {
        if let Some((obj, r)) = scene.hit(&ray, 0.001, std::f64::INFINITY) {
            if let Some(scatter) = obj.material.scatter(&ray, &r) {
                attenuation = Color(attenuation.0 * scatter.attenuation.0);
                ray = scatter.direction;
            } else {
                attenuation = Color::default();
            }
        } else {
            return Color(attenuation.0 * skybox(&ray).0);
        }
        max_depth -= 1
    }
    Color::default()
}

struct Frame<'a> {
    camera: &'a Camera,
    pos: &'a CameraPosition,
    scene: &'a Scene,
    samples: usize,
}

fn render_image(args: &clap::ArgMatches) -> Result<()> {
    use rayon::prelude::*;

    let width = args.value_of("width").unwrap().parse::<usize>()?;
    let output = args.value_of("output").unwrap();
    let use_random = args.is_present("random");
    let mut samples = args.value_of("samples").unwrap().parse::<usize>()?;
    if samples == 0 {
        log::warn!("samples set to 0, using 1");
        samples = 1;
    }

    let mut camera = camera::CameraBuilder::default();
    camera
        .vfov(20.0)
        .focal_length(1.0)
        .aperture(0.1)
        .width(width)
        .aspect_ratio((3, 2));
    let camera = camera.build()?;
    let mut pos = CameraPosition::look_at(
        Point(Vec3::new(13.0, 2.0, 3.0)),
        Point(Vec3::new(0.0, 0.0, 0.0)),
        Vec3::new(0.0, 1.0, 0.0),
    );
    pos.focus_length = 10.0;

    log::trace!("Camera: {:?}", camera);
    log::trace!("Pos: {:?}", pos);

    let mut image = image::Image::from_dimm(camera.dimm);
    image.samples = samples;
    let scene = if use_random {
        random_scene()
    } else {
        create_scene()
    };

    let frame = Frame {
        camera: &camera,
        pos: &pos,
        scene: &scene,
        samples,
    };

    log::trace!("render");

    let scanlines = image.scanlines_mut();

    if args.is_present("single_threaded") {
        let mut rng = rand::thread_rng();
        scanlines
            .into_iter()
            .enumerate()
            .for_each(|(line_idx, buf)| {
                render_scanline(&frame, &mut rng, line_idx, buf);
            })
    } else {
        scanlines.into_par_iter().enumerate().for_each_init(
            || rand::thread_rng(),
            |rng, (line_idx, buf)| {
                render_scanline(&frame, rng, line_idx, buf);
            },
        );
    }

    image::to_image(&image, output);
    Ok(())
}

fn render_scanline(frame: &Frame<'_>, rng: &mut ThreadRng, line_idx: usize, buf: &mut [Color]) {
    use rand::prelude::*;
    for i in 0..frame.camera.dimm.width {
        let mut c = Color::default();
        for _ in 0..frame.samples {
            let r = frame.camera.pixel_ray(
                &frame.pos,
                i as f64 + rng.gen::<f64>(),
                line_idx as f64 + rng.gen::<f64>(),
            );
            c += ray_color(&r, &frame.scene, 50);
        }
        buf[i] = c
    }
}

fn random_scene() -> Scene {
    use rand::prelude::*;
    let mut scene = Scene::default();

    let mat_ground = Lambertian {
        albedo: Color(Vec3::new(0.5, 0.5, 0.5)),
    };
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, -1000.0, 0.0)),
            radius: 1000.0,
        },
        mat_ground,
    );

    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, 1.0, 0.0)),
            radius: 1.0,
        },
        Dielectric { ir: 1.5 },
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(-4.0, 1.0, 0.0)),
            radius: 1.0,
        },
        Lambertian {
            albedo: Color(Vec3::new(0.4, 0.2, 0.1)),
        },
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(4.0, 1.0, 0.0)),
            radius: 1.0,
        },
        Metal::new(Color(Vec3::new(0.7, 0.6, 0.5)), None),
    );

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let mat_select = rng.gen::<f64>();
            let radius = random_real(&mut rng, 0.15, 0.25);
            let center = Point(Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                radius,
                b as f64 + 0.9 * rng.gen::<f64>(),
            ));
            let keepout = Vec3::new(3.0, radius, 0.0);
            if (center.0 - keepout).length() <= 0.9 {
                continue;
            }
            let sphere = Sphere { center, radius };
            if mat_select < 0.7 {
                // diffuse
                let albedo = Color(Vec3::random() * Vec3::random());
                scene.add(sphere, Lambertian { albedo });
            } else if mat_select < 0.95 {
                // metal
                let albedo = Color(Vec3::random_range(0.5, 1.0));
                let fuzz = random_real(&mut rng, 0.0, 0.5);
                scene.add(sphere, Metal::new(albedo, Some(fuzz)));
            } else {
                // glass
                scene.add(sphere, Dielectric { ir: 1.5 });
            }
        }
    }

    scene
}

fn create_scene() -> Scene {
    let mut scene = Scene::default();

    let mat_ground = Lambertian {
        albedo: Color(Vec3::new(0.8, 0.8, 0.0)),
    };
    let mat_center = Lambertian {
        albedo: Color(Vec3::new(0.1, 0.2, 0.5)),
    };
    let mat_left = Dielectric { ir: 1.5 };
    let mat_right = Metal::new(Color(Vec3::new(0.8, 0.6, 0.2)), Some(0.0));

    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, -100.5, -1.0)),
            radius: 100.0,
        },
        mat_ground,
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, 0.0, -1.0)),
            radius: 0.5,
        },
        mat_center,
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(-1.0, 0.0, -1.0)),
            radius: 0.5,
        },
        mat_left.clone(),
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(-1.0, 0.0, -1.0)),
            radius: -0.4,
        },
        mat_left,
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(1.0, 0.0, -1.0)),
            radius: 0.5,
        },
        mat_right,
    );
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
                    )
                    .arg(
                        clap::Arg::with_name("random")
                            .long("random")
                            .help("use random scene"),
                    )
                    .arg(
                        clap::Arg::with_name("single_threaded")
                            .long("single-threaded")
                            .help("render on a single core"),
                    ),
            )
            .get_matches()
    }
}
