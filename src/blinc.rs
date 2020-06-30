use crate::config::Config;
use anyhow::Result;
use log::debug;
use std::env;
use std::process;
use transition::Transition;

pub(crate) struct Blinc {
    config: Config,
}

impl Blinc {
    pub(crate) fn new(config: Config) -> Result<Self> {
        Blinc::init(&config)?;
        Ok(Self { config })
    }

    pub(crate) fn init(config: &Config) -> Result<()> {
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

    pub(crate) fn exec_tasks(&self) -> Result<()> {
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
