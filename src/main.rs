use anyhow::Result;

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
