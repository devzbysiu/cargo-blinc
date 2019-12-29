use clap::crate_authors;
use clap::crate_version;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use config::Config;
use log::debug;
use std::process;
use transition::Transition;

mod colors;
mod config;
mod task;

#[cfg(test)]
mod testutils;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let arguments = parse_args();
    if arguments.is_present("init") {
        debug!("init argument passed, initializing config");
        Config::default().store()?;
        process::exit(0);
    }
    let config = Config::get()?;
    let tx = transition(&config)?.start()?;
    for task in config.tasks() {
        debug!("executing {:?}", task);
        if !task.run()?.success() {
            tx.notify_failure()?;
            debug!("task failed, exiting");
            process::exit(1);
        }
    }
    tx.notify_success()?;
    Ok(())
}

fn parse_args<'a>() -> ArgMatches<'a> {
    let arguments = App::new("blinc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Blinks USB notifier light with different colors depending on command exit code")
        // this subcommand is only because of how cargo runs custom commands:
        // cargo blinc --init == cargo-blinc blinc --init
        .subcommand(
            App::new("blinc")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Blinks USB notifier light with different colors depending on command exit code")
            .arg(
                Arg::with_name("init")
                    .help("Initializes configuration file named .blinc (note the dot)")
                    .short("i")
                    .long("init"),
            ),
        )
        .get_matches();
    let arguments = arguments
        .subcommand_matches("blinc")
        .expect("blinc subcommand should be present");
    debug!("arguments: {:?}", arguments);
    arguments.clone()
}

fn transition(config: &Config) -> Result<Transition, failure::Error> {
    Ok(Transition::from(config.pending())
        .on_success(config.success())
        .on_failure(config.failure()))
}
