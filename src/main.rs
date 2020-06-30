//! This binary allows to run predefined tasks and notify you about status of those tasks using LED
//! light [blink(1)](https://blink1.thingm.com/).

#![deny(missing_docs)]

use anyhow::Result;
use args::Opt;
use config::Config;
use log::debug;
use std::env;
use std::process;
use structopt::StructOpt;
use transition::Transition;

mod args;
mod colors;
mod config;
mod task;

#[cfg(test)]
mod testutils;

fn main() -> Result<()> {
    env_logger::init();
    let Opt::Blinc { init, config } = Opt::from_args();
    if init.is_some() {
        debug!("init argument passed, initializing config");
        // can unwrap because it's checked earlier
        Config::default().store(init.unwrap())?;
        process::exit(0);
    }
    Blinc::new(Config::get(config)?)?.exec_tasks()?;
    Ok(())
}

struct Blinc {
    config: Config,
}

impl Blinc {
    fn new(config: Config) -> Result<Self> {
        Blinc::init(&config)?;
        Ok(Self { config })
    }

    fn init(config: &Config) -> Result<()> {
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

    fn exec_tasks(&self) -> Result<()> {
        let tx = transition(&self.config)?.start()?;
        for task in self.config.tasks() {
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
}

fn transition(config: &Config) -> Result<Transition> {
    Ok(Transition::new(config.pending())
        .on_success(config.success())
        .on_failure(config.failure()))
}
