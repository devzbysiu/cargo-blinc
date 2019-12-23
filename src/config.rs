use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

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
        let mut config = String::new();
        File::open(".blink").and_then(|mut f| f.read_to_string(&mut config))?;
        let mut config: Config = toml::from_str(&config)?;
        let command = config.command;
        let split_command = command.split(' ').collect::<Vec<&str>>();
        config.command = Config::read_command(&split_command);
        config.args = Some(
            split_command
                .iter()
                .skip(1)
                .map(|s| s.to_string())
                .collect(),
        );
        Ok(config)
    }

    fn read_command(split_command: &Vec<&str>) -> String {
        split_command.get(0).unwrap_or(&"echo").to_string()
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
