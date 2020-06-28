//! This binary allows to run predefined tasks and notify you about status of those tasks using LED
//! light [blink(1)](https://blink1.thingm.com/).

#![deny(missing_docs)]

use anyhow::Result;
use args::parse_args;
use clap::ArgMatches;
use config::Config;
use log::debug;
use std::env;
use std::process;
use transition::Transition;

mod args;
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

#[allow(clippy::needless_pass_by_value)]
fn init_config(args: ArgMatches<'_>) -> Result<()> {
    let init_path = args
        .value_of("init")
        .expect("no path specified for init subcommand");
    Config::default().store(init_path)?;
    Ok(())
}

fn handle_env_variables(config: &Config) -> Result<()> {
    if let Some(env) = config.env() {
        debug!("setting up env variables");
        env.iter().for_each(|(k, v)| {
            debug!("setting {} = {}", k, v);
            env::set_var(k, v);
        })
    } else {
        debug!("no env variables to set");
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
