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
    pub mod lighting;
    pub mod material;
    pub mod perlin;
    pub mod rect;
    pub mod scene;
    pub mod skybox;
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
use objects::scene::Scene;
use rand::prelude::ThreadRng;
use util::{math::random_real, Color, Point, Ray, Vec3};

use self::{
    argparse::RenderSettings,
    objects::{
        perlin::NoiseTexture,
        rect::{xy_rect, xz_rect, yz_rect, RectBox},
        texture::ConstantTexture,
    },
};
use crate::{
    objects::{
        lighting::{DiffuseLight, FairyLight},
        material::{Dielectric, Lambertian, Metal},
        scene::SceneBuilder,
        sphere::Sphere,
        texture::CheckerTexture,
    },
    texture::ImageTexture,
};

const DEFAULT_WIDTH: &str = "640";
const DEFAULT_SAMPLES: &str = "100";
const DEFAULT_REFLECT_DEPTH: &str = "50";
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
            argparse::Render::BoxLight(args) => render_boxlight(args),
            argparse::Render::Cornell(args) => render_cornell_box(args),
        },
        argparse::SubCommand::Test(sub) => run_test(sub),
    }
    .map_err(|e| {
        log::error!("{:?}", e);
        anyhow::anyhow!("unrecoverable {} failure", clap::crate_name!())
    })
}

fn ray_color(
    hit_stack: &mut Vec<usize>,
    incoming: &Ray,
    scene: &Scene,
    mut max_depth: usize,
) -> Color {
    let mut ray = *incoming;
    let mut attenuation = Color::ones();
    let mut emitted = Color::default();

    let mut workspace = scene.workspace_scene(hit_stack);

    while max_depth > 0 {
        if let Some((obj, r)) = workspace.hit_workspace(&ray, 0.001, std::f64::INFINITY) {
            if let Some(e) = obj.material.emitted(&ray, &r) {
                emitted += Color(attenuation.0 * e.0);
            }
            if let Some(scatter) = obj.material.scatter(&ray, &r) {
                attenuation = Color(attenuation.0 * scatter.attenuation.0);
                ray = scatter.direction;
            } else {
                break;
            }
        } else {
            emitted += Color(attenuation.0 * scene.skybox.background(&ray).0);
            break;
        }
        max_depth -= 1
    }
    emitted
}

struct Frame<'a> {
    camera: &'a Camera,
    pos: &'a CameraPosition,
    scene: &'a Scene,
}

fn run_test(_args: &argparse::Test) -> Result<()> {
    log::error!("there is nothing to test!");
    Ok(())
}
fn render_random(args: &argparse::RenderRandom) -> Result<()> {
    let scene = random_scene(args.night);
    // 1170 x 2532
    // let width = args.config.width;
    // let mut camera = camera::CameraBuilder::default();
    // camera
    //     .vfov(40.0)
    //     .focal_length(1.0)
    //     .aperture(0.001)
    //     .width(width)
    //     .aspect_ratio((1170, 2532));

    // let camera = camera.build()?;
    // let mut pos = CameraPosition::look_at(
    //     Point(Vec3::new(13.0, 2.0, 3.0)),
    //     Point(Vec3::new(0.0, -1.0, 0.0)),
    //     Vec3::new(0.0, 1.0, 0.0),
    // );
    // pos.focus_length = 10.0;
    let (camera, pos) = default_camera(&args.config)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

fn render_demo(args: &argparse::RenderDemo) -> Result<()> {
    let scene = create_scene();
    let (camera, pos) = default_camera(&args.config)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

fn render_perlin(args: &argparse::RenderPerlin) -> Result<()> {
    let scene = create_perlin_demo();
    let (camera, pos) = default_camera(&args.config)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

fn render_earth(args: &argparse::RenderEarth) -> Result<()> {
    let scene = create_earth_demo()?;
    let (camera, pos) = default_camera(&args.config)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

fn render_boxlight(args: &argparse::RenderBoxLight) -> Result<()> {
    let scene = create_box_light();
    let (camera, pos) = default_camera(&args.config)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

fn render_cornell_box(args: &argparse::RenderCornellBox) -> Result<()> {
    let scene = create_cornell_box();

    let width = args.config.width;
    let mut camera = camera::CameraBuilder::default();
    camera
        .vfov(40.0)
        .focal_length(1.0)
        .aperture(0.00001)
        .width(width)
        .aspect_ratio((1, 1));

    let camera = camera.build()?;
    let mut pos = CameraPosition::look_at(
        Point(Vec3::new(278.0, 278.0, -800.0)),
        Point(Vec3::new(278.0, 278.0, 0.0)),
        Vec3::new(0.0, 1.0, 0.0),
    );
    pos.focus_length = 10.0;

    render_scene(&args.config, &scene, &camera, &pos)
}

fn default_camera(args: &RenderSettings) -> Result<(Camera, CameraPosition)> {
    let width = args.width;
    let mut camera = camera::CameraBuilder::default();
    camera
        .vfov(20.0)
        .focal_length(1.0)
        .aperture(0.01)
        .width(width)
        .aspect_ratio((3, 2));

    let camera = camera.build()?;
    let mut pos = CameraPosition::look_at(
        Point(Vec3::new(13.0, 2.0, 3.0)),
        Point(Vec3::new(0.0, 0.0, 0.0)),
        Vec3::new(0.0, 1.0, 0.0),
    );
    pos.focus_length = 10.0;
    Ok((camera, pos))
}

fn render_scene(
    args: &argparse::RenderSettings,
    scene: &Scene,
    camera: &Camera,
    pos: &CameraPosition,
) -> Result<()> {
    use rayon::prelude::*;

    let output = args.output.as_str();

    let samples = if args.samples == 0 {
        log::warn!("samples set to 0, using 1");
        1
    } else {
        args.samples
    };

    log::trace!("Camera: {:?}", camera);
    log::trace!("Pos: {:?}", pos);

    let mut image = image::Image::from_dimm(camera.dimm);
    image.samples = samples;

    let frame = Frame { camera, pos, scene };

    log::trace!("render");

    let scanlines = image.scanlines_mut();
    let max_depth = args.max_reflect;

    let count = std::sync::atomic::AtomicUsize::new(0);
    let total = scanlines.len();
    if args.single_threaded {
        let mut rng = rand::thread_rng();
        let mut hit_stack = Vec::new();
        scanlines
            .iter_mut()
            .enumerate()
            .for_each(|(line_idx, buf)| {
                render_scanline(
                    &frame,
                    &mut rng,
                    samples,
                    max_depth,
                    &mut hit_stack,
                    line_idx,
                    buf,
                );
                let x = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                log::debug!("render line {}/{}", x + 1, total);
            })
    } else {
        scanlines.into_par_iter().enumerate().for_each_init(
            || (rand::thread_rng(), Vec::<usize>::new()),
            // rand::thread_rng,
            |(rng, hit_stack), (line_idx, buf)| {
                render_scanline(&frame, rng, samples, max_depth, hit_stack, line_idx, buf);
                let x = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                log::debug!("render line {}/{}", x + 1, total);
            },
        );
    }

    image::to_image(&image, output);
    Ok(())
}

fn render_scanline(
    frame: &Frame<'_>,
    rng: &mut ThreadRng,
    samples: usize,
    max_depth: usize,
    hit_stack: &mut Vec<usize>,
    line_idx: usize,
    buf: &mut [Color],
) {
    use rand::prelude::*;
    for (idx, buf_c) in buf.iter_mut().enumerate() {
        let mut c = Color::default();
        for _ in 0..samples {
            let r = frame.camera.pixel_ray(
                frame.pos,
                idx as f64 + rng.gen::<f64>(),
                line_idx as f64 + rng.gen::<f64>(),
            );
            c += ray_color(hit_stack, &r, frame.scene, max_depth);
        }
        *buf_c = c
    }
}

fn random_scene(night: bool) -> Scene {
    use rand::prelude::*;
    let mut scene = SceneBuilder::default();
    if night {
        scene.set_skybox(objects::skybox::SkyBox::None);
    }

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

    if night {
        scene.add(
            Sphere {
                center: util::Point(Vec3::new(-4.0, 1.0, 0.0)),
                radius: 1.0,
            },
            FairyLight::new(ConstantTexture::from(Color(
                Vec3::new(0.7, 0.6, 0.5).scale(1.3),
            ))),
        );
    } else {
        scene.add(
            Sphere {
                center: util::Point(Vec3::new(-4.0, 1.0, 0.0)),
                radius: 1.0,
            },
            Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.4, 0.2, 0.1)))),
        );
    }
    scene.add(
        Sphere {
            center: util::Point(Vec3::new(4.0, 1.0, 0.0)),
            radius: 1.0,
        },
        Metal::new(Color(Vec3::new(0.7, 0.6, 0.5)), None),
    );

    // scene.add(
    //     yz_rect(0.0, 2.0, -0.0, 3.0, -8.0),
    //     Metal::new(Color(Vec3::new(0.7, 0.6, 0.5)), None),
    // );

    let mut rng = thread_rng();

    #[derive(Clone, Copy)]
    enum BallTypes {
        Color,
        SphereLight,
        Glass,
        Metal,
        Checker,
        Marble,
    }

    let light_weight = if night { 4.0 } else { 0.0 };

    let types = [
        (BallTypes::Color, 4.0),
        (BallTypes::SphereLight, light_weight),
        (BallTypes::Glass, 1.0),
        (BallTypes::Metal, 4.0),
        (BallTypes::Checker, 0.3),
        (BallTypes::Marble, 0.0),
    ];

    for a in -11..11 {
        for b in -11..11 {
            let item = types.choose_weighted(&mut rng, |x| x.1).unwrap().0;

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
            match item {
                BallTypes::Color => {
                    let albedo = Color(Vec3::random() * Vec3::random());
                    scene.add(sphere, Lambertian::new(ConstantTexture::from(albedo)));
                }
                BallTypes::SphereLight => {
                    let albedo = Color((Vec3::random() * Vec3::random()).scale(5.0));
                    scene.add(sphere, FairyLight::new(ConstantTexture::from(albedo)));
                }
                BallTypes::Glass => {
                    // glass
                    scene.add(sphere, Dielectric { ir: 1.5 });
                }
                BallTypes::Metal => {
                    // metal
                    let albedo = Color(Vec3::random_range(0.5, 1.0));
                    let fuzz = random_real(&mut rng, 0.0, 0.5);
                    scene.add(sphere, Metal::new(albedo, Some(fuzz)));
                }
                BallTypes::Checker => {
                    // checker
                    let checker_color = Color(Vec3::random() * Vec3::random());
                    let checker_texture = CheckerTexture::new(
                        8.0 / radius,
                        ConstantTexture::from(checker_color),
                        ConstantTexture::from(Color(Vec3::new(0.9, 0.9, 0.9))),
                    );
                    scene.add(sphere, Lambertian::new(checker_texture));
                }
                BallTypes::Marble => {
                    // marble
                    let mat = Lambertian::new(NoiseTexture::scale(16.0));
                    scene.add(sphere, mat);
                }
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

fn create_cornell_box() -> Scene {
    let mut scene = SceneBuilder::default();
    scene.set_skybox(objects::skybox::SkyBox::None);

    let red = Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.65, 0.05, 0.05))));
    let white = Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.73, 0.73, 0.73))));
    let green = Lambertian::new(ConstantTexture::from(Color(Vec3::new(0.12, 0.45, 0.15))));
    let light = FairyLight::new(ConstantTexture::from(Color(Vec3::new(15.0, 15.0, 15.0))));

    let box_size = 555.0;
    scene.add(yz_rect(0.0, box_size, 0.0, box_size, box_size), green);
    scene.add(yz_rect(0.0, box_size, 0.0, box_size, 0.0), red);
    scene.add(xz_rect(213.0, 343.0, 227.0, 332.0, 554.0), light);
    scene.add(xz_rect(0.0, box_size, 0.0, box_size, 0.0), white.clone());
    scene.add(
        xz_rect(0.0, box_size, 0.0, box_size, box_size),
        white.clone(),
    );
    scene.add(
        xy_rect(0.0, box_size, 0.0, box_size, box_size),
        white.clone(),
    );

    scene.add(
        RectBox::new(
            Point(Vec3::new(130.0, 0.0, 65.0)),
            Point(Vec3::new(295.0, 165.0, 230.0)),
        ),
        white.clone(),
    );

    scene.add(
        RectBox::new(
            Point(Vec3::new(265.0, 0.0, 295.0)),
            Point(Vec3::new(430.0, 330.0, 460.0)),
        ),
        white,
    );

    scene.finalize()
}
fn create_box_light() -> Scene {
    let mut scene = SceneBuilder::default();
    scene.set_skybox(objects::skybox::SkyBox::None);

    let mat = Lambertian::new(NoiseTexture::scale(4.0));

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
            center: util::Point(Vec3::new(0.0, 2.0, -0.0)),
            radius: 2.0,
        },
        mat,
    );
    // scene.add(
    //     XYRect {
    //         x0: 3.0,
    //         x1: 5.0,
    //         y0: 1.0,
    //         y1: 3.0,
    //         k: -2.0,
    //     },
    //     DiffuseLight::new(ConstantTexture::from(Color(Vec3::new(4.0, 4.0, 4.0)))),
    // );

    scene.add(
        // xy_rect(3.0, 5.0, 1.0, 3.0, -2.0),
        // yz_rect(3.0, 5.0, 1.0, 3.0, -2.0),
        xz_rect(3.0, 5.0, 1.0, 3.0, 3.5),
        DiffuseLight::new(ConstantTexture::from(Color(Vec3::new(4.0, 4.0, 4.0)))),
    );
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
