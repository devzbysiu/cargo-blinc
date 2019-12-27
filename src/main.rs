use config::Config;
use std::process::Command;
use std::process::ExitStatus;
use transition::Transition;

mod config;

fn main() -> Result<(), failure::Error> {
    let config = Config::init()?;
    let tx = Transition::from(config.pending())
        .on_success(config.success())
        .on_failure(config.failure())
        .run()?;
    if run(config.command(), config.args())?.success() {
        tx.notify_success()?;
    } else {
        tx.notify_failure()?;
    }
    Ok(())
}

fn run(cmd: &str, args: Vec<String>) -> Result<ExitStatus, failure::Error> {
    Ok(Command::new(cmd).args(args).status()?)
}
