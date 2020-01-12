use anyhow::Result;
use clap::crate_authors;
use clap::crate_version;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use config::Config;
use log::debug;
use std::env;
use std::process;
use transition::Transition;

mod colors;
mod config;
mod task;

#[cfg(test)]
mod testutils;

fn main() -> Result<()> {
    env_logger::init();
    let arguments = parse_args();
    if arguments.is_present("init") {
        debug!("init argument passed, initializing config");
        init_config(arguments)?;
        process::exit(0);
    }
    let config_path = arguments
        .value_of("config")
        .expect("no config option passed");
    let config = Config::get(config_path)?;
    handle_env_variables(&config)?;
    handle_tasks_execution(&config)?;
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
                    .long("init")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("config")
                    .help("Points to configuration file")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .default_value(".blinc"),
            ),
        )
        .get_matches();
    let arguments = arguments
        .subcommand_matches("blinc")
        .expect("blinc subcommand should be present");
    debug!("arguments: {:?}", arguments);
    arguments.clone()
}

fn init_config<'a>(args: ArgMatches<'a>) -> Result<()> {
    let init_path = args
        .value_of("init")
        .expect("no path specified for init subcommand");
    Config::default().store(init_path)?;
    Ok(())
}

fn handle_env_variables(config: &Config) -> Result<()> {
    if let Some(env) = config.env() {
        env.iter().for_each(|(k, v)| env::set_var(k, v))
    }
    Ok(())
}

fn handle_tasks_execution(config: &Config) -> Result<()> {
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

fn transition(config: &Config) -> Result<Transition> {
    Ok(Transition::new(config.pending())
        .on_success(config.success())
        .on_failure(config.failure()))
}
