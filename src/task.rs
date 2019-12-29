use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::process::Command;
use std::process::ExitStatus;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task {
    cmd: String,
    args: Option<Vec<String>>,
}

impl Task {
    pub(crate) fn new(cmd: &str, args: &[&str]) -> Self {
        Self {
            cmd: cmd.to_string(),
            args: Some(args.iter().map(|&arg| arg.to_string()).collect()),
        }
    }

    pub(crate) fn command(&self) -> &str {
        &self.cmd
    }

    pub(crate) fn args(&self) -> Vec<String> {
        self.args.clone().unwrap_or_else(|| vec![])
    }

    pub(crate) fn run(&self) -> Result<ExitStatus, failure::Error> {
        Ok(Command::new(self.command()).args(self.args()).status()?)
    }
}
