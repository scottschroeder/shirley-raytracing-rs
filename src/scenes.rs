use anyhow::Result;
use rand::{thread_rng, Rng};
use raytracer::{
    camera::{Camera, CameraBuilder, CameraPosition},
    core::{math::random_real, Color, Point, Vec3},
    geometry::{
        rect::{xy_rect, xz_rect, yz_rect, RectBox},
        sphere::Sphere,
    },
    material::{
        dielectric::Dielectric,
        lambertian::Lambertian,
        lighting::{DiffuseLight, FairyLight},
        metal::Metal,
        texture::loader::TextureLoader,
    },
    scene::{Scene, SceneBuilder},
    skybox::SkyBox,
};

use super::{argparse, argparse::CameraSettings, render_scene};

pub fn create_cornell_box() -> anyhow::Result<Scene> {
    let mut scene = SceneBuilder::default();
    scene.set_skybox(SkyBox::None);

    let red = Lambertian::new(TextureLoader::solid(0.65, 0.05, 0.05));
    let white = Lambertian::new(TextureLoader::solid(0.73, 0.73, 0.73));
    let green = Lambertian::new(TextureLoader::solid(0.12, 0.45, 0.15));
    let light = FairyLight::new(TextureLoader::solid(15.0, 15.0, 15.0));

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

pub fn create_perlin_demo() -> anyhow::Result<Scene> {
    let mut scene = SceneBuilder::default();

    create_ground_checker(&mut scene);
    let mat = Lambertian::new(TextureLoader::noise(4.0));

    scene.add(
        Sphere {
            center: Point(Vec3::new(0.0, 2.0, -0.0)),
            radius: 2.0,
        },
        mat,
    );
    scene.finalize()
}

pub fn create_earth_demo() -> anyhow::Result<Scene> {
    let mut scene = SceneBuilder::default();

    create_ground_checker(&mut scene);
    scene.add(
        Sphere {
            center: Point(Vec3::new(4.0, 1.0, 1.0)),
            radius: 1.0,
        },
        Lambertian::new(TextureLoader::EarthBuiltin),
    );
    scene.finalize()
}
pub fn create_box_light() -> anyhow::Result<Scene> {
    let mut scene = SceneBuilder::default();
    scene.set_skybox(SkyBox::None);

    create_ground_checker(&mut scene);

    let mat = Lambertian::new(TextureLoader::noise(4.0));

    scene.add(
        Sphere {
            center: Point(Vec3::new(0.0, 2.0, -0.0)),
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
        DiffuseLight::new(TextureLoader::solid(4.0, 4.0, 4.0)),
    );
    scene.finalize()
}
pub fn render_saved(args: &argparse::RenderSaved) -> Result<()> {
    let f = std::fs::File::open(args.scene_input.as_str())?;
    let scene: SceneBuilder = serde_json::from_reader(f)?;
    let scene = scene.finalize()?;
    let (camera, pos) = default_camera(&args.camera)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

pub fn render_random(args: &argparse::RenderRandom) -> Result<()> {
    let mut rng = thread_rng();
    let scene = random_scene(&mut rng, args.night);

    if let Some(save) = args.scene_output.as_ref() {
        let mut f = std::fs::File::create(save)?;
        serde_json::to_writer_pretty(&mut f, &scene)?;
    }

    let scene = scene.finalize()?;
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
    let (camera, pos) = default_camera(&args.camera)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

pub fn render_demo(args: &argparse::RenderDemo) -> Result<()> {
    let scene = create_scene()?;
    let (camera, pos) = default_camera(&args.camera)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

pub fn render_perlin(args: &argparse::RenderPerlin) -> Result<()> {
    let scene = create_perlin_demo()?;
    let (camera, pos) = default_camera(&args.camera)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

pub fn render_earth(args: &argparse::RenderEarth) -> Result<()> {
    let scene = create_earth_demo()?;
    let (camera, pos) = default_camera(&args.camera)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

pub fn render_boxlight(args: &argparse::RenderBoxLight) -> Result<()> {
    let scene = create_box_light()?;
    let (camera, pos) = default_camera(&args.camera)?;
    render_scene(&args.config, &scene, &camera, &pos)
}

pub fn render_cornell_box(args: &argparse::RenderCornellBox) -> Result<()> {
    let scene = create_cornell_box()?;

    let width = args.camera.width;
    let mut camera = CameraBuilder::default();
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

pub fn default_camera(args: &CameraSettings) -> Result<(Camera, CameraPosition)> {
    let mut camera = CameraBuilder::default();
    camera
        .vfov(args.camera_fov)
        .focal_length(args.camera_focal_length)
        .aperture(args.camera_aperture)
        .width(args.width)
        .aspect_ratio(args.camera_aspect_ratio.ratio());

    let camera = camera.build()?;
    let mut pos = CameraPosition::look_at(
        Point(Vec3::new(13.0, 2.0, 3.0)),
        Point(Vec3::new(0.0, 0.0, 0.0)),
        Vec3::new(0.0, 1.0, 0.0),
    );
    pos.focus_length = 10.0;
    Ok((camera, pos))
}

pub fn create_ground_checker(scene: &mut SceneBuilder) {
    let ground_texture = TextureLoader::checker(
        10.0,
        TextureLoader::solid(0.2, 0.3, 0.1),
        TextureLoader::solid(0.9, 0.9, 0.9),
    );
    let mat_ground = Lambertian::new(ground_texture);
    let rect = 30.0;
    scene.add(xz_rect(-rect, rect, -rect, rect, -0.0001), mat_ground);
    // scene.add(
    //     Sphere {
    //         center: Point(Vec3::new(0.0, -1000.0, 0.0)),
    //         radius: 1000.0,
    //     },
    //     mat_ground,
    // );
}

pub fn create_fancy_ground(scene: &mut SceneBuilder) {
    const RECT_SIZE: f64 = 30.0;
    const TOP_COAT_DEPTH: f64 = 0.01;
    const LAYER_SEP: f64 = 0.01;

    let lower_surface = Lambertian::new(TextureLoader::checker(
        3.0,
        TextureLoader::noise(1.0),
        TextureLoader::solid(0.1, 0.1, 0.1),
    ));

    scene.add(
        xz_rect(
            -RECT_SIZE,
            RECT_SIZE,
            -RECT_SIZE,
            RECT_SIZE,
            -TOP_COAT_DEPTH - LAYER_SEP,
        ),
        lower_surface,
    );
    scene.add(
        RectBox::new(
            Point(Vec3::new(-RECT_SIZE, -TOP_COAT_DEPTH, -RECT_SIZE)),
            Point(Vec3::new(RECT_SIZE, 0.0, RECT_SIZE)),
        ),
        Dielectric { ir: 1.0 },
    );
}

pub fn random_scene<R: Rng>(rng: &mut R, night: bool) -> SceneBuilder {
    use rand::prelude::*;
    let mut scene = SceneBuilder::default();
    if night {
        scene.set_skybox(SkyBox::None);
    }
    if night {
        create_ground_checker(&mut scene);
    } else {
        create_fancy_ground(&mut scene);
    }

    let mut balls: Vec<Sphere> = Vec::new();

    let mut check_fit_ball = |mut s: Sphere| {
        let orig = s.radius;
        for other in &balls {
            let dist = (other.center.0 - s.center.0).length();
            let rem = dist - other.radius;
            s.radius = std::cmp::min_by(s.radius, rem, |a, b| a.total_cmp(b));
        }
        let delta = orig - s.radius;
        s.center = Point(s.center.0 - Vec3::new(0.0, delta, 0.0));
        balls.push(s.clone());
        s
    };

    scene.add(
        check_fit_ball(Sphere {
            center: Point(Vec3::new(0.0, 1.0, 0.0)),
            radius: 1.0,
        }),
        Dielectric { ir: 1.5 },
    );

    if night {
        scene.add(
            check_fit_ball(Sphere {
                center: Point(Vec3::new(-4.0, 1.0, 0.0)),
                radius: 1.0,
            }),
            FairyLight::new(TextureLoader::solid_from_vec(
                Vec3::new(0.7, 0.6, 0.5).scale(1.3),
            )),
        );
    } else {
        scene.add(
            check_fit_ball(Sphere {
                center: Point(Vec3::new(-4.0, 1.0, 0.0)),
                radius: 1.0,
            }),
            Lambertian::new(TextureLoader::solid(0.4, 0.2, 0.1)),
        );
    }
    scene.add(
        check_fit_ball(Sphere {
            center: Point(Vec3::new(4.0, 1.0, 0.0)),
            radius: 1.0,
        }),
        Metal::new(Color(Vec3::new(0.7, 0.6, 0.5)), None),
    );

    // scene.add(
    //     yz_rect(0.0, 2.0, -0.0, 3.0, -8.0),
    //     Metal::new(Color(Vec3::new(0.7, 0.6, 0.5)), None),
    // );

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
            let item = types.choose_weighted(rng, |x| x.1).unwrap().0;

            let radius = random_real(rng, 0.05, 0.25);
            let center = Point(Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                radius,
                b as f64 + 0.9 * rng.gen::<f64>(),
            ));
            let keepout = Vec3::new(3.0, radius, 0.0);
            if (center.0 - keepout).length() <= 0.9 {
                continue;
            }
            let sphere = check_fit_ball(Sphere { center, radius });
            match item {
                BallTypes::Color => {
                    let albedo = rng.gen::<Vec3>() * rng.gen::<Vec3>();
                    scene.add(
                        sphere,
                        Lambertian::new(TextureLoader::solid_from_vec(albedo)),
                    );
                }
                BallTypes::SphereLight => {
                    let albedo = (rng.gen::<Vec3>() * rng.gen::<Vec3>()).scale(5.0);
                    scene.add(
                        sphere,
                        FairyLight::new(TextureLoader::solid_from_vec(albedo)),
                    );
                }
                BallTypes::Glass => {
                    // glass
                    scene.add(sphere, Dielectric { ir: 1.5 });
                }
                BallTypes::Metal => {
                    // metal
                    let albedo = Color(Vec3::random_range_with_rng(rng, 0.5, 1.0));
                    let fuzz = random_real(rng, 0.0, 0.5);
                    scene.add(sphere, Metal::new(albedo, Some(fuzz)));
                }
                BallTypes::Checker => {
                    // checker
                    let checker_color = rng.gen::<Vec3>() * rng.gen::<Vec3>();
                    let checker_texture = TextureLoader::checker(
                        8.0 / radius,
                        TextureLoader::solid_from_vec(checker_color),
                        TextureLoader::solid(0.9, 0.9, 0.9),
                    );
                    scene.add(sphere, Lambertian::new(checker_texture));
                }
                BallTypes::Marble => {
                    // marble
                    let mat = Lambertian::new(TextureLoader::noise(16.0));
                    scene.add(sphere, mat);
                }
            }
        }
    }

    scene
}

pub fn create_scene() -> anyhow::Result<Scene> {
    let mut scene = SceneBuilder::default();

    let mat_ground = Lambertian::new(TextureLoader::solid(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(TextureLoader::solid(0.1, 0.2, 0.5));
    let mat_left = Dielectric { ir: 1.5 };
    let mat_right = Metal::new(Color(Vec3::new(0.8, 0.6, 0.2)), Some(0.0));

    scene.add(
        Sphere {
            center: Point(Vec3::new(0.0, -100.5, -1.0)),
            radius: 100.0,
        },
        mat_ground,
    );
    scene.add(
        Sphere {
            center: Point(Vec3::new(0.0, 0.0, -1.0)),
            radius: 0.5,
        },
        mat_center,
    );
    scene.add(
        Sphere {
            center: Point(Vec3::new(-1.0, 0.0, -1.0)),
            radius: 0.5,
        },
        mat_left.clone(),
    );
    scene.add(
        Sphere {
            center: Point(Vec3::new(-1.0, 0.0, -1.0)),
            radius: -0.4,
        },
        mat_left,
    );
    scene.add(
        Sphere {
            center: Point(Vec3::new(1.0, 0.0, -1.0)),
            radius: 0.5,
        },
        mat_right,
    );
    // scene.add(objects::sphere::Sphere {
    //     center: Point(Vec3::new(-1.0, 0.0, -1.0)),
    //     radius: 0.3,
    // });
    // scene.add(objects::sphere::Sphere {
    //     center: Point(Vec3::new(1.0, 0.0, -1.0)),
    //     radius: 0.3,
    // });
    scene.finalize()
}
