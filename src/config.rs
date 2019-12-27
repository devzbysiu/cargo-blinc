use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

const COMMAND_NAME: usize = 1;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    command: String,
    pending: String,
    args: Option<Vec<String>>,
    failure: String,
    success: String,
}

impl Config {
    pub(crate) fn init() -> Result<Config, failure::Error> {
        Config::read_config(&mut File::open(".blink")?)
    }

    pub(crate) fn read_config<R: Read>(read: &mut R) -> Result<Config, failure::Error> {
        Ok(init_config(read_config(read)?))
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }

    pub(crate) fn pending(&self) -> &str {
        &self.pending
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

fn read_config<R: Read>(read: &mut R) -> Result<Config, failure::Error> {
    let mut config_contents = String::new();
    read.read_to_string(&mut config_contents)?;
    let c: Config = toml::from_str(&config_contents)?;
    Ok(c)
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
    use std::io;

    #[test]
    fn test_config_with_valid_config() -> Result<(), failure::Error> {
        let config_contents = r#"
          command = "cargo test"
          pending = "blue white"
          failure = "red"
          success = "green"
        "#
        .to_string();

        let c = Config::read_config(&mut ReaderMock::new(config_contents))?;

        assert_eq!(c.pending(), "blue white", "Testing transition");
        assert_eq!(c.command(), "cargo", "Testing command");
        assert_eq!(c.args(), vec!["test"], "Testing command arguments");
        assert_eq!(c.failure(), "red", "Testing failure color");
        assert_eq!(c.success(), "green", "Testing success color");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_transition_key() {
        let config_contents = r#"
          command = "cargo test"
          failure = "red"
          success = "green"
        "#
        .to_string();
        Config::read_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_command_key() {
        let config_contents = r#"
          pending = "blue white"
          failure = "red"
          success = "green"
        "#
        .to_string();
        Config::read_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_failure_key() {
        let config_contents = r#"
          command = "cargo test"
          pending = "blue white"
          success = "green"
        "#
        .to_string();
        Config::read_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_success_key() {
        let config_contents = r#"
          command = "cargo test"
          pending = "blue white"
          failure = "red"
        "#
        .to_string();
        Config::read_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    struct ReaderMock {
        contents: String,
    }

    impl ReaderMock {
        fn new(contents: String) -> ReaderMock {
            ReaderMock { contents }
        }
    }

    impl Read for ReaderMock {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Ok(1)
        }

        fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
            self.contents.as_bytes().read_to_string(buf)?;
            Ok(buf.len())
        }
    }
}
