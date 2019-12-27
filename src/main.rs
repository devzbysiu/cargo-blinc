use config::Config;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use transition::Transition;

mod config;

fn main() -> Result<(), failure::Error> {
    let config = config()?;
    let tx = transition(&config)?.start()?;
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

fn transition(config: &Config) -> Result<Transition, failure::Error> {
    Ok(Transition::from(config.pending())
        .on_success(config.success())
        .on_failure(config.failure()))
}

fn run<I: Into<String>>(cmd: I, args: Vec<String>) -> Result<ExitStatus, failure::Error> {
    Ok(Command::new(cmd.into()).args(args).status()?)
}
