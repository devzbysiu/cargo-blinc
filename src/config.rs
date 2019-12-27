use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    tasks: Vec<Task>,
    colors: Colors,
}

impl Config {
    pub(crate) fn load() -> Result<Self, failure::Error> {
        Self::read(&mut File::open(".blinc")?)
    }

    pub(crate) fn read<R: Read>(read: &mut R) -> Result<Self, failure::Error> {
        Ok(read_config(read)?)
    }

    pub(crate) fn store(&self) -> Result<(), failure::Error> {
        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(".blinc")?;
        self.write(&mut config_file)?;
        Ok(())
    }

    pub(crate) fn write<W: Write>(&self, write: &mut W) -> Result<(), failure::Error> {
        write.write_all(toml::to_string(&self)?.as_bytes())?;
        Ok(())
    }

    pub(crate) fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub(crate) fn pending(&self) -> &Vec<String> {
        &self.colors.pending
    }

    pub(crate) fn failure(&self) -> &str {
        &self.colors.failure
    }

    pub(crate) fn success(&self) -> &str {
        &self.colors.success
    }
}

fn read_config<R: Read>(read: &mut R) -> Result<Config, failure::Error> {
    let mut config_content = String::new();
    read.read_to_string(&mut config_content)?;
    let c: Config = toml::from_str(&config_content)?;
    Ok(c)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tasks: vec![Task {
                cmd: "cargo".to_string(),
                args: Some(vec!["test".to_string()]),
            }],
            colors: Colors {
                pending: vec!["blue".to_string(), "white".to_string()],
                failure: "red".to_string(),
                success: "green".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Colors {
    pending: Vec<String>,
    failure: String,
    success: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task {
    cmd: String,
    args: Option<Vec<String>>,
}

impl Task {
    pub(crate) fn command(&self) -> &str {
        &self.cmd
    }

    pub(crate) fn args(&self) -> Vec<String> {
        self.args.clone().unwrap_or_else(|| vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils::*;

    #[test]
    fn test_load_config_with_valid_config() -> Result<(), failure::Error> {
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
            "Testing command"
        );
        assert_eq!(
            c.tasks().first().unwrap().args(),
            vec!["check"],
            "Testing command arguments"
        );
        assert_eq!(c.failure(), "red", "Testing failure color");
        assert_eq!(c.success(), "green", "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_command_config_with_lack_of_cmd_key() {
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
    fn test_command_config_with_lack_of_optional_args_key() -> Result<(), failure::Error> {
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
            "Testing command"
        );
        assert_eq!(
            c.tasks().first().unwrap().args(),
            Vec::<String>::new(),
            "Testing command arguments"
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
    fn test_store_config() -> Result<(), failure::Error> {
        let config_content = r#"[[tasks]]
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
