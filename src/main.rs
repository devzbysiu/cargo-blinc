use config::Config;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use transition::Transition;
use transition::Transmitter;

mod config;

fn main() -> Result<(), failure::Error> {
    let config = config()?;
    let tx = transition_transmitter(&config)?;
    if run(config.command(), config.args())?.success() {
        tx.notify_success()?;
    } else {
        tx.notify_failure()?;
    }
    Ok(())
}

fn config() -> Result<Config, failure::Error> {
    let config_path = Path::new(".blinc");
    let mut config = Config::default();
    if config_path.exists() {
        config = Config::load()?;
    }
    Ok(config)
}

fn transition_transmitter(config: &Config) -> Result<Transmitter, failure::Error> {
    Ok(Transition::from(config.pending())
        .on_success(config.success())
        .on_failure(config.failure())
        .run()?)
}

fn run(cmd: &str, args: Vec<String>) -> Result<ExitStatus, failure::Error> {
    Ok(Command::new(cmd).args(args).status()?)
}
