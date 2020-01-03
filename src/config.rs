use crate::colors::Colors;
use crate::task::Task;
use anyhow::Result;
use log::debug;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

const FILE_NAME: &str = ".blinc";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    tasks: Vec<Task>,
    colors: Colors,
}

impl Config {
    pub(crate) fn get() -> Result<Self> {
        if Path::new(".blinc").exists() {
            debug!("config file exists, loading");
            Ok(Self::load()?)
        } else {
            debug!("no config file, using default configuration");
            Ok(Self::default())
        }
    }

    fn load() -> Result<Self> {
        Self::read(&mut File::open(FILE_NAME)?)
    }

    fn read<R: Read>(read: &mut R) -> Result<Self> {
        Ok(read_config(read)?)
    }

    pub(crate) fn store(&self) -> Result<()> {
        debug!(
            "storing config: {:?} under path {:?}",
            self,
            env::current_dir()?
        );
        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(FILE_NAME)?;
        self.write(&mut config_file)?;
        Ok(())
    }

    fn write<W: Write>(&self, write: &mut W) -> Result<()> {
        write.write_all(toml::to_string(&self)?.as_bytes())?;
        debug!("config written");
        Ok(())
    }

    pub(crate) fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub(crate) fn pending(&self) -> &Vec<String> {
        self.colors.pending()
    }

    pub(crate) fn failure(&self) -> &str {
        self.colors.failure()
    }

    pub(crate) fn success(&self) -> &str {
        self.colors.success()
    }
}

fn read_config<R: Read>(read: &mut R) -> Result<Config> {
    let mut config_content = String::new();
    read.read_to_string(&mut config_content)?;
    debug!("read config {}", config_content);
    let config = toml::from_str(&config_content)?;
    debug!("created config struct: {:?}", config);
    Ok(config)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task::new("cargo", &["check"]),
                Task::new("cargo", &["test"]),
            ],
            colors: Colors::new(&["blue", "white"], "red", "green"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils::*;

    #[test]
    fn test_load_config_with_valid_config() -> Result<()> {
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "white"]
            failure = "red"
            success = "green"
        "#
        .to_string();

        let c = Config::read(&mut ReaderStub::new(config_content))?;

        assert_eq!(c.pending()[0], "blue", "Testing transition");
        assert_eq!(c.pending()[1], "white", "Testing transition");
        assert_eq!(
            c.tasks().first().unwrap().command(),
            "cargo",
            "Testing first task command"
        );
        assert_eq!(
            c.tasks().first().unwrap().args(),
            vec!["check"],
            "Testing first task arguments"
        );
        assert_eq!(c.failure(), "red", "Testing failure color");
        assert_eq!(c.success(), "green", "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_tasks_config_with_lack_of_cmd_key() {
        let config_content = r#"
            [[tasks]]
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "white"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tasks_config_with_empty_tasks_key() {
        let config_content = r#"
            [[tasks]]

            [colors]
            pending = ["blue", "white"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tasks_config_with_lack_of_tasks() {
        let config_content = r#"
            [colors]
            pending = ["blue", "white"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    fn test_tasks_config_with_lack_of_optional_args_key() -> Result<()> {
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"

            [[tasks]]
            cmd = "cargo"

            [colors]
            pending = ["blue", "white"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        let c = Config::read(&mut ReaderStub::new(config_content))?;

        assert_eq!(c.pending()[0], "blue", "Testing transition");
        assert_eq!(c.pending()[1], "white", "Testing transition");
        assert_eq!(
            c.tasks().first().unwrap().command(),
            "cargo",
            "Testing first task command"
        );
        assert_eq!(
            c.tasks().first().unwrap().args(),
            Vec::<String>::new(),
            "Testing first task arguments"
        );
        assert_eq!(c.failure(), "red", "Testing failure color");
        assert_eq!(c.success(), "green", "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_pending_key() {
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_failure_key() {
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "white"]
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_success_key() {
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "white"]
            failure = "red"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_colors_key() {
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    fn test_store_config() -> Result<()> {
        let config_content = r#"[[tasks]]
cmd = "cargo"
args = ["check"]

[[tasks]]
cmd = "cargo"
args = ["test"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
"#
        .to_string();

        let mut writer = WriterMock::new(&config_content);
        Config::default().write(&mut writer)?;

        assert_eq!(true, writer.all_config_written(), "Testing writing config");

        Ok(())
    }
}
