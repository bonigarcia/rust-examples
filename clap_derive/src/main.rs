use clap::Parser;
use log::LevelFilter::{Debug, Info, Trace};
use std::io::Write;
use env_logger::fmt::Color;
use log::Level;

/// Simple program using --debug and --trace
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Display DEBUG messages
    #[clap(short, long)]
    debug: bool,

    /// Display TRACE messages
    #[clap(short, long)]
    trace: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut filter = match cli.debug {
        true => Debug,
        false => Info,
    };
    if cli.trace {
        filter = Trace
    }

    env_logger::Builder::new()
        .filter_level(filter)
        .format(|buf, record| {
            let mut level_style = buf.style();
            match record.level() {
                Level::Trace => level_style.set_color(Color::Cyan),
                Level::Debug => level_style.set_color(Color::Blue),
                Level::Info => level_style.set_color(Color::Green),
                Level::Warn => level_style.set_color(Color::Yellow),
                Level::Error => level_style.set_color(Color::Red).set_bold(true),
            };
            writeln!(
                buf,
                "{}\t{}",
                level_style.value(record.level()),
                record.args()
            )
        })
        .target(env_logger::Target::Stdout)
        .init();

    log::error!("Error message");
    log::info!("Info message");
    log::debug!("Debug message");
    log::trace!("Trace message");
}