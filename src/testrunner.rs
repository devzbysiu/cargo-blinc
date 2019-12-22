use std::process::Command;
use std::process::ExitStatus;

pub fn run_tests() -> Result<ExitStatus, failure::Error> {
    Ok(Command::new("cargo").arg("test").status()?)
}
