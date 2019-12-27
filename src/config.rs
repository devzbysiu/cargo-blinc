use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    command: Command,
    colors: Colors,
}

impl Config {
    pub(crate) fn load() -> Result<Config, failure::Error> {
        Config::read(&mut File::open(".blinc")?)
    }

    pub(crate) fn read<R: Read>(read: &mut R) -> Result<Config, failure::Error> {
        Ok(read_config(read)?)
    }

    pub(crate) fn store(&self) -> Result<(), failure::Error> {
        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(".blinc")?;
        Config::write(&mut config_file, &Config::default())?;
        Ok(())
    }

    pub(crate) fn write<W: Write>(write: &mut W, config: &Config) -> Result<(), failure::Error> {
        write.write_all(toml::to_string(&config)?.as_bytes())?;
        Ok(())
    }

    pub(crate) fn command(&self) -> &str {
        &self.command.cmd
    }

    pub(crate) fn args(&self) -> Vec<String> {
        self.command.args.clone().unwrap_or(vec![])
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
        Config {
            command: Command {
                cmd: "cargo".to_string(),
                args: Some(vec!["test".to_string()]),
            },
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

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    cmd: String,
    args: Option<Vec<String>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    #[test]
    fn test_load_config_with_valid_config() -> Result<(), failure::Error> {
        let config_content = r#"
            [command]
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
        assert_eq!(c.command(), "cargo", "Testing command");
        assert_eq!(c.args(), vec!["test"], "Testing command arguments");
        assert_eq!(c.failure(), "red", "Testing failure color");
        assert_eq!(c.success(), "green", "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_command_config_with_lack_of_cmd_key() {
        let config_content = r#"
            [command]
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
            [command]
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
        assert_eq!(c.command(), "cargo", "Testing command");
        assert_eq!(c.args(), Vec::<String>::new(), "Testing command arguments");
        assert_eq!(c.failure(), "red", "Testing failure color");
        assert_eq!(c.success(), "green", "Testing success color");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_colors_config_with_lack_of_pending_key() {
        let config_content = r#"
            [command]
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
            [command]
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
            [command]
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
        let config_content = r#"[command]
cmd = "cargo"
args = ["test"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
"#
        .to_string();

        let mut writer = WriterMock::new(&config_content);
        Config::write(&mut writer, &Config::default())?;

        assert_eq!(true, writer.all_config_written(), "Testing writing config");

        Ok(())
    }

    struct ReaderStub {
        contents: String,
    }

    impl ReaderStub {
        fn new(contents: String) -> ReaderStub {
            ReaderStub { contents }
        }
    }

    impl Read for ReaderStub {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Ok(1)
        }

        fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
            self.contents.as_bytes().read_to_string(buf)?;
            Ok(buf.len())
        }
    }

    struct WriterMock {
        wrote_content: String,
        expected_content: String,
    }

    impl WriterMock {
        fn new<I: Into<String>>(expected_content: I) -> Self {
            WriterMock {
                wrote_content: "".to_string(),
                expected_content: expected_content.into(),
            }
        }

        fn all_config_written(&self) -> bool {
            self.wrote_content == self.expected_content
        }
    }

    impl Write for WriterMock {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(1)
        }

        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            self.wrote_content = String::from_utf8(buf.to_vec()).unwrap();
            Ok(())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
