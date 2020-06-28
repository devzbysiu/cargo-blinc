use crate::colors::Colors;
use crate::task::Task;
use anyhow::Result;
use log::debug;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use transition::Led;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    tasks: Vec<Task>,
    colors: Colors,
    env: Option<HashMap<String, String>>,
}

impl Config {
    pub(crate) fn get<A: AsRef<Path>>(path: A) -> Result<Self> {
        let path = path.as_ref();
        if Path::new(path).exists() {
            debug!("config file exists, loading from path {:?}", path);
            Ok(Self::load(path)?)
        } else {
            debug!("no config file, using default configuration");
            Ok(Self::default())
        }
    }

    fn load<A: AsRef<Path>>(path: A) -> Result<Self> {
        Self::read(&mut File::open(path.as_ref())?)
    }

    fn read<R: Read>(read: &mut R) -> Result<Self> {
        Ok(read_config(read)?)
    }

    pub(crate) fn store<A: AsRef<Path>>(&self, path: A) -> Result<()> {
        debug!(
            "storing config: {:?} under path {:?} with name {:?}",
            self,
            env::current_dir()?,
            path.as_ref()
        );
        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
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

    pub(crate) fn env(&self) -> &Option<HashMap<String, String>> {
        &self.env
    }

    pub(crate) fn pending(&self) -> &[Led] {
        self.colors.pending()
    }

    pub(crate) fn failure(&self) -> &Led {
        self.colors.failure()
    }

    pub(crate) fn success(&self) -> &Led {
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
            colors: Colors::new(vec![Led::Blue, Led::Blank], Led::Red, Led::Green),
            env: Some(HashMap::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::testutils::{init_logger, ReaderStub, WriterMock};
    use crate::Result;
    use transition::Led;

    #[test]
    fn test_load_config_with_valid_config() -> Result<()> {
        init_logger();
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"
        "#
        .to_string();

        let c = Config::read(&mut ReaderStub::new(config_content))?;

        assert_eq!(c.pending()[0], Led::Blue, "Testing transition");
        assert_eq!(c.pending()[1], Led::Blank, "Testing transition");
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
        assert_eq!(c.failure(), &Led::Red, "Testing failure color");
        assert_eq!(c.success(), &Led::Green, "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_tasks_config_with_lack_of_cmd_key() {
        init_logger();
        let config_content = r#"
            [[tasks]]
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tasks_config_with_empty_tasks_key() {
        init_logger();
        let config_content = r#"
            [[tasks]]

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tasks_config_with_lack_of_tasks() {
        init_logger();
        let config_content = r#"
            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    fn test_tasks_config_with_lack_of_optional_args_key() -> Result<()> {
        init_logger();
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"

            [[tasks]]
            cmd = "cargo"

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"
        "#
        .to_string();
        let c = Config::read(&mut ReaderStub::new(config_content))?;

        assert_eq!(c.pending()[0], Led::Blue, "Testing transition");
        assert_eq!(c.pending()[1], Led::Blank, "Testing transition");
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
        assert_eq!(c.failure(), &Led::Red, "Testing failure color");
        assert_eq!(c.success(), &Led::Green, "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_pending_key() {
        init_logger();
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
        init_logger();
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "blank"]
            success = "green"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_success_key() {
        init_logger();
        let config_content = r#"
            [[tasks]]
            cmd = "cargo"
            args = ["check"]

            [[tasks]]
            cmd = "cargo"
            args = ["test"]

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
        "#
        .to_string();
        Config::read(&mut ReaderStub::new(config_content)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_colors_key() {
        init_logger();
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
        init_logger();
        let config_content = r#"[[tasks]]
cmd = "cargo"
args = ["check"]

[[tasks]]
cmd = "cargo"
args = ["test"]

[colors]
pending = ["blue", "blank"]
failure = "red"
success = "green"

[env]
"#
        .to_string();

        let mut writer = WriterMock::new(&config_content);
        Config::default().write(&mut writer)?;

        assert_eq!(true, writer.all_config_written(), "Testing writing config");

        Ok(())
    }
}
