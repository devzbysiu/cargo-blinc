use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

const COMMAND_NAME: usize = 1;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    transition: String,
    command: String,
    args: Option<Vec<String>>,
    failure: String,
    success: String,
}

impl Config {
    pub(crate) fn init() -> Result<Config, failure::Error> {
        Ok(init_config(read_config()?))
    }

    pub(crate) fn transition(&self) -> &str {
        &self.transition
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }

    pub(crate) fn args(&self) -> Vec<String> {
        self.args.clone().unwrap_or(vec![])
    }

    pub(crate) fn failure(&self) -> &str {
        &self.failure
    }

    pub(crate) fn success(&self) -> &str {
        &self.success
    }
}

fn read_config() -> Result<Config, failure::Error> {
    let mut config = String::new();
    File::open(".blink").and_then(|mut f| f.read_to_string(&mut config))?;
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

fn init_config(config: Config) -> Config {
    let mut config = config;
    let command = config.command;
    let command_and_args = command.split(' ').collect::<Vec<&str>>();
    config.command = read_command_name(&command_and_args);
    config.args = read_args(&command_and_args);
    config
}

fn read_command_name(split_command: &Vec<&str>) -> String {
    split_command.first().unwrap_or(&"echo").to_string()
}

fn read_args(split_command: &Vec<&str>) -> Option<Vec<String>> {
    Some(
        split_command
            .iter()
            .skip(COMMAND_NAME)
            .map(|s| s.to_string())
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_with_valid_config_file() -> Result<(), failure::Error> {
        let c = Config::init()?;
        assert_eq!(
            c.transition(),
            "blue white",
            "Testing transition of pre-execution section"
        );
        assert_eq!(c.command(), "cargo", "Testing command of execution section");
        assert_eq!(
            c.args(),
            vec!["test"],
            "Testing command arguments of execution section"
        );
        Ok(())
    }
}
