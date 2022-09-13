use clap::Parser;

/// Simple program using --debug and --trace
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Display DEBUG messages
    #[clap(short, long)]
    debug: bool,

    /// Display TRACE messages
    #[clap(short, long)]
    trace: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello {}!", cli.name);

    let mut filter = match cli.debug {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };
    if cli.trace {
        filter = log::LevelFilter::Trace
    }

    env_logger::Builder::new()
        .filter_level(filter)
        .init();

    log::error!("Error message");
    log::info!("Info message");
    log::debug!("Debug message");
    log::trace!("Trace message");
}