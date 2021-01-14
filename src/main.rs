use anyhow::Result;

pub mod util {
    mod color;
    mod vec3;
    pub use color::Color;
    pub use vec3::Vec3;
}
pub mod image;

use util::{Color, Vec3};

fn main() -> Result<()> {
    color_backtrace::install();
    let args = cli::get_args();
    cli::setup_logger(args.occurrences_of("verbosity"));
    log::trace!("Args: {:?}", args);

    match args.subcommand() {
        ("test", Some(sub_m)) => test_fn(sub_m),
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

fn test_fn(_args: &clap::ArgMatches) -> Result<()> {
    use std::io::Write;

    let image_width = 256;
    let image_height = 256;

    let stdout = std::io::stdout();
    let mut output = stdout.lock();

    write!(output, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        log::trace!("scanlines remaining: {}", j);
        for i in 0..image_width {
            let r = (i as f64) / (image_width - 1) as f64;
            let g = (j as f64) / (image_height - 1) as f64;
            let b = 0.25f64;
            let c = Color(Vec3::new(r, g, b));

            image::write_ppm_pixel(&mut output, &c)?;
            writeln!(output, "")?;
        }
    }
    log::info!("done");

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
            .subcommand(SubCommand::with_name("test"))
            .get_matches()
    }
}
