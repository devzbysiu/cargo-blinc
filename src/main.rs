#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

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

#[cfg(test)]
mod testutils;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    if parse_args().is_present("init") {
        debug!("init argument passed, initializing config");
        Config::default().store()?;
        process::exit(0);
    }
    let config = config()?;
    let tx = transition(&config)?.start()?;
    for task in config.tasks() {
        debug!("executing {:?}", task);
        if !run(task.command(), task.args())?.success() {
            tx.notify_failure()?;
            debug!("task failed, exiting");
            process::exit(1);
        }
    }
    tx.notify_success()?;
    Ok(())
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("blinc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Blinks USB notifier light with different colors depending on command exit code")
        .arg(
            Arg::with_name("init")
                .help("Initializes configuration file named .blinc (note the dot)")
                .short("i")
                .long("init"),
        )
        // this argument is only because of how cargo runs custom commands:
        // cargo blinc --init == cargo-blinc blinc --init
        .arg(Arg::with_name("blinc"))
        .get_matches()
}

fn config() -> Result<Config, failure::Error> {
    if Path::new(".blinc").exists() {
        debug!("config file exists, loading");
        Ok(Config::load()?)
    } else {
        debug!("no config file, using default configuration");
        Ok(Config::default())
    }
}

fn transition(config: &Config) -> Result<Transition, failure::Error> {
    Ok(Transition::from(config.pending())
        .on_success(config.success())
        .on_failure(config.failure()))
}

fn run<I: Into<String>>(cmd: I, args: Vec<String>) -> Result<ExitStatus, failure::Error> {
    Ok(Command::new(cmd.into()).args(args).status()?)
}
