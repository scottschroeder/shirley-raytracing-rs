pub mod util {
    mod color;
    pub(crate) mod fp;
    pub mod math;
    mod vec3;
    pub use color::Color;
    pub use vec3::{Dimm, Point, Ray, Vec3, EACH_DIMM};
}
pub mod camera;
pub mod objects {
    mod aabb;
    mod bbox_tree;
    mod hittable;
    pub mod material;
    pub mod perlin;
    pub mod scene;
    pub mod sphere;
    pub mod texture;
    pub use aabb::Aabb;
    pub use hittable::{Geometry, Hittable};
}
pub mod image;

pub mod texture;

mod argparse;

use anyhow::Result;
use camera::{Camera, CameraPosition};
use objects::{scene::Scene, Hittable};
use rand::prelude::ThreadRng;
use util::{math::random_real, Color, Point, Ray, Vec3};

use self::objects::{perlin::NoiseTexture, texture::ConstantTexture};
use crate::{
    objects::{
        material::{Dielectric, Lambertian, Metal},
        scene::SceneBuilder,
        sphere::Sphere,
        texture::CheckerTexture,
    },
    texture::ImageTexture,
};

const DEFAULT_WIDTH: &str = "640";
const DEFAULT_SAMPLES: &str = "100";
const DEFAULT_OUTPUT: &str = "out.png";
const EARTH_TEXTURE: &[u8] = include_bytes!("../assets/earthmap.jpg");

fn main() -> Result<()> {
    color_backtrace::install();
    let args = argparse::get_args();
    setup_logger(args.verbose);
    log::trace!("Args: {:?}", args);

    match &args.subcmd {
        argparse::SubCommand::Render(sub) => match sub {
            argparse::Render::Random(args) => render_random(args),
            argparse::Render::Demo(args) => render_demo(args),
            argparse::Render::Perlin(args) => render_perlin(args),
            argparse::Render::Earth(args) => render_earth(args),
        },
        argparse::SubCommand::Test(sub) => run_test(sub),
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

fn run_test(_args: &argparse::Test) -> Result<()> {
    log::error!("there is nothing to test!");
    Ok(())
}

fn render_random(args: &argparse::RenderRandom) -> Result<()> {
    let scene = random_scene();
    render_scene(&args.config, &scene)
}

fn render_demo(args: &argparse::RenderDemo) -> Result<()> {
    let scene = create_scene();
    render_scene(&args.config, &scene)
}

fn render_perlin(args: &argparse::RenderPerlin) -> Result<()> {
    let scene = create_perlin_demo();
    render_scene(&args.config, &scene)
}

fn render_earth(args: &argparse::RenderEarth) -> Result<()> {
    let scene = create_earth_demo()?;
    render_scene(&args.config, &scene)
}

fn render_scene(args: &argparse::RenderSettings, scene: &Scene) -> Result<()> {
    use rayon::prelude::*;

    let width = args.width;
    let output = args.output.as_str();

    let samples = if args.samples == 0 {
        log::warn!("samples set to 0, using 1");
        1
    } else {
        args.samples
    };

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

    let frame = Frame {
        camera: &camera,
        pos: &pos,
        scene: &scene,
        samples,
    };

    log::trace!("render");

    let scanlines = image.scanlines_mut();

    let count = std::sync::atomic::AtomicUsize::new(0);
    let total = scanlines.len();
    if args.single_threaded {
        let mut rng = rand::thread_rng();
        scanlines
            .iter_mut()
            .enumerate()
            .for_each(|(line_idx, buf)| {
                render_scanline(&frame, &mut rng, line_idx, buf);
                let x = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                log::debug!("render line {}/{}", x + 1, total);
            })
    } else {
        scanlines.into_par_iter().enumerate().for_each_init(
            rand::thread_rng,
            |rng, (line_idx, buf)| {
                render_scanline(&frame, rng, line_idx, buf);
                let x = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                log::debug!("render line {}/{}", x + 1, total);
            },
        );
    }

    image::to_image(&image, output);
    Ok(())
}

fn render_scanline(frame: &Frame<'_>, rng: &mut ThreadRng, line_idx: usize, buf: &mut [Color]) {
    use rand::prelude::*;
    for (idx, buf_c) in buf.iter_mut().enumerate() {
        let mut c = Color::default();
        for _ in 0..frame.samples {
            let r = frame.camera.pixel_ray(
                frame.pos,
                idx as f64 + rng.gen::<f64>(),
                line_idx as f64 + rng.gen::<f64>(),
            );
            c += ray_color(&r, frame.scene, 50);
        }
        *buf_c = c
    }
}

fn random_scene() -> Scene {
    use rand::prelude::*;
    let mut scene = SceneBuilder::default();

    // let mat_ground = Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.5, 0.5, 0.5))));
    let ground_texture = CheckerTexture::new(
        10.0,
        ConstantTexture::from(Color(Vec3::new(0.2, 0.3, 0.1))),
        ConstantTexture::from(Color(Vec3::new(0.9, 0.9, 0.9))),
    );
    let mat_ground = Lambertian::new(ground_texture);
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
        Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.4, 0.2, 0.1)))),
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
            let radius = random_real(&mut rng, 0.05, 0.25);
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
            if mat_select < 0.5 {
                // diffuse
                let albedo = Color(Vec3::random() * Vec3::random());
                scene.add(sphere, Lambertian::new(ConstantTexture::from(albedo)));
            } else if mat_select < 0.6 {
                // checker
                let checker_color = Color(Vec3::random() * Vec3::random());
                let checker_texture = CheckerTexture::new(
                    8.0 / radius,
                    ConstantTexture::from(checker_color),
                    ConstantTexture::from(Color(Vec3::new(0.9, 0.9, 0.9))),
                );
                scene.add(sphere, Lambertian::new(checker_texture));
            } else if mat_select < 0.7 {
                // marble
                let mat = Lambertian::new(NoiseTexture::scale(16.0));
                scene.add(sphere, mat);
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

    scene.finalize()
}

fn create_scene() -> Scene {
    let mut scene = SceneBuilder::default();

    let mat_ground = Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.8, 0.8, 0.0))));
    let mat_center = Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.1, 0.2, 0.5))));
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
    scene.finalize()
}

fn create_perlin_demo() -> Scene {
    let mut scene = SceneBuilder::default();

    let mat = Lambertian::new(NoiseTexture::scale(4.0));

    let ground_texture = CheckerTexture::new(
        10.0,
        ConstantTexture::from(Color(Vec3::new(0.2, 0.3, 0.1))),
        ConstantTexture::from(Color(Vec3::new(0.9, 0.9, 0.9))),
    );
    let mat_ground = Lambertian::new(ground_texture);

    log::debug!("{:?}", mat);

    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, -1000.0, -0.0)),
            radius: 1000.0,
        },
        mat_ground,
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, 2.0, -0.0)),
            radius: 2.0,
        },
        mat,
    );
    scene.finalize()
}

fn create_earth_demo() -> anyhow::Result<Scene> {
    let mut scene = SceneBuilder::default();

    let earth_texture = ImageTexture::load_from_memory(EARTH_TEXTURE)?;
    let mat = Lambertian::new(earth_texture);

    let ground_texture = CheckerTexture::new(
        10.0,
        ConstantTexture::from(Color(Vec3::new(0.2, 0.3, 0.1))),
        ConstantTexture::from(Color(Vec3::new(0.9, 0.9, 0.9))),
    );
    let mat_ground = Lambertian::new(ground_texture);

    scene.add(
        Sphere {
            center: util::Point(Vec3::new(0.0, -1000.0, -0.0)),
            radius: 1000.0,
        },
        mat_ground,
    );
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(4.0, 1.0, 1.0)),
            radius: 1.0,
        },
        mat,
    );
    Ok(scene.finalize())
}

pub fn setup_logger(level: u8) {
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
