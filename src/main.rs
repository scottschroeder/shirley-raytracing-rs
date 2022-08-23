mod argparse;
use anyhow::Result;
use raytracer::{
    bvh::bbox_tree::BboxTreeWorkspace,
    camera::{Camera, CameraPosition},
    image,
    render::{render_scanline, Frame},
    scene::Scene,
};
mod scenes;

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

fn main() -> Result<()> {
    color_backtrace::install();
    let args = argparse::get_args();
    setup_logger(args.verbose);
    log::trace!("Args: {:?}", args);

    match &args.subcmd {
        argparse::SubCommand::Render(sub) => match sub {
            argparse::Render::Random(args) => scenes::render_random(args),
            argparse::Render::Demo(args) => scenes::render_demo(args),
            argparse::Render::Perlin(args) => scenes::render_perlin(args),
            argparse::Render::Earth(args) => scenes::render_earth(args),
            argparse::Render::BoxLight(args) => scenes::render_boxlight(args),
            argparse::Render::Cornell(args) => scenes::render_cornell_box(args),
            argparse::Render::Saved(args) => scenes::render_saved(args),
        },
        argparse::SubCommand::Test(sub) => run_test(sub),
    }
    .map_err(|e| {
        log::error!("{:?}", e);
        anyhow::anyhow!("unrecoverable {} failure", clap::crate_name!())
    })
}

fn run_test(_args: &argparse::Test) -> Result<()> {
    log::error!("there is nothing to test!");
    Ok(())
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
        // let mut rng = rand::prng::chacha::ChaChaRng;
        let mut hit_stack = BboxTreeWorkspace::default();
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
            || (rand::thread_rng(), BboxTreeWorkspace::default()),
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
