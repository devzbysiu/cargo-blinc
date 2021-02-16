//! This binary allows to run predefined tasks and notify you about status of those tasks using LED
//! light [blink(1)](https://blink1.thingm.com/).

#![deny(missing_docs)]

use anyhow::Result;
use args::Opt;
use blinc::Blinc;
use config::Config;
use log::debug;
use std::process;
use structopt::StructOpt;

mod args;
mod blinc;
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
    Blinc::new(Config::get(config)?).exec_tasks()?;
    Ok(())
}
