use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

const COMMAND_NAME: usize = 1;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    command: String,
    args: Option<Vec<String>>,
    pending: Vec<String>,
    failure: String,
    success: String,
}

impl Config {
    pub(crate) fn load() -> Result<Config, failure::Error> {
        Config::load_config(&mut File::open(".blinc")?)
    }

    pub(crate) fn load_config<R: Read>(read: &mut R) -> Result<Config, failure::Error> {
        Ok(load_config(read_config(read)?))
    }

    pub(crate) fn store(&self) -> Result<(), failure::Error> {
        let default_config = Config::default();
        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(".blinc")?;
        config_file.write_all(toml::to_string(&default_config)?.as_bytes())?;
        Ok(())
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }

    pub(crate) fn pending(&self) -> &Vec<String> {
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

fn load_config(config: Config) -> Config {
    let mut config = config;
    let command = config.command;
    let command_and_args = command.split(' ').collect::<Vec<&str>>();
    config.command = read_command_name(&command_and_args);
    config.args = read_args(&command_and_args);
    config
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

fn read_command_name(split_command: &Vec<&str>) -> String {
    split_command.first().unwrap_or(&"echo").to_string()
}

fn read_config<R: Read>(read: &mut R) -> Result<Config, failure::Error> {
    let mut config_contents = String::new();
    read.read_to_string(&mut config_contents)?;
    let c: Config = toml::from_str(&config_contents)?;
    Ok(c)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            command: "cargo test".to_string(),
            pending: vec!["blue".to_string(), "white".to_string()],
            args: None,
            failure: "red".to_string(),
            success: "green".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    #[test]
    fn test_config_with_valid_config() -> Result<(), failure::Error> {
        let config_contents = r#"
          command = "cargo test"
          pending = ["blue", "white"]
          failure = "red"
          success = "green"
        "#
        .to_string();

        let c = Config::load_config(&mut ReaderMock::new(config_contents))?;

        assert_eq!(c.pending()[0], "blue", "Testing transition");
        assert_eq!(c.pending()[1], "white", "Testing transition");
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
        Config::load_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_command_key() {
        let config_contents = r#"
          pending = ["blue", "white"]
          failure = "red"
          success = "green"
        "#
        .to_string();
        Config::load_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_failure_key() {
        let config_contents = r#"
          command = "cargo test"
          pending = ["blue", "white"]
          success = "green"
        "#
        .to_string();
        Config::load_config(&mut ReaderMock::new(config_contents)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_config_with_lack_of_success_key() {
        let config_contents = r#"
          command = "cargo test"
          pending = ["blue", "white"]
          failure = "red"
        "#
        .to_string();
        Config::load_config(&mut ReaderMock::new(config_contents)).unwrap();
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
