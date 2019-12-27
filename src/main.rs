#[macro_use]
extern crate clap;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use config::Config;
use std::path::Path;
use std::process;
use std::process::Command;
use std::process::ExitStatus;
use transition::Transition;

mod config;

fn main() -> Result<(), failure::Error> {
    let args = parse_args();
    if args.is_present("init") {
        Config::default().store()?;
        process::exit(0);
    }
    let config = config()?;
    let tx = transition(&config)?.start()?;
    if run(config.command(), config.args())?.success() {
        tx.notify_success()?;
    } else {
        tx.notify_failure()?;
    }
    Ok(())
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("blinc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Blinks USB notifier light with different colors depending on command exit code")
        .arg(
            Arg::with_name("init")
                .help("Initializes configuration file .blinc (note the dot)")
                .short("i")
                .long("init"),
        )
        .arg(Arg::with_name("blinc"))
        .get_matches()
}

fn config() -> Result<Config, failure::Error> {
    let config_path = Path::new(".blinc");
    let mut config = Config::default();
    if config_path.exists() {
        config = Config::load()?;
    }
    Ok(config)
}

fn transition(config: &Config) -> Result<Transition, failure::Error> {
    Ok(Transition::from(config.pending())
        .on_success(config.success())
        .on_failure(config.failure()))
}

fn run<I: Into<String>>(cmd: I, args: Vec<String>) -> Result<ExitStatus, failure::Error> {
    Ok(Command::new(cmd.into()).args(args).status()?)
}
