use crate::config::Config;
use anyhow::Result;
use log::debug;
use std::env;
use std::process;
use transition::Transition;

pub(crate) struct Blinc {
    config: Config,
}

impl Blinc {
    pub(crate) fn new(config: Config) -> Self {
        Blinc::init(&config);
        Self { config }
    }

    pub(crate) fn init(config: &Config) {
        if let Some(env) = config.env() {
            debug!("setting up env variables");
            for (k, v) in env {
                debug!("setting {} = {}", k, v);
                env::set_var(k, v);
            }
        } else {
            debug!("no env variables to set");
        }
    }

    pub(crate) fn exec_tasks(&self) -> Result<()> {
        let tx = transition(&self.config)?.start()?;
        for task in self.config.tasks() {
            debug!("executing {:?}", task);
            if !task.run()?.success() {
                tx.notify_failure()?;
                debug!("task failed, exiting");
                process::exit(1);
            }
        }
        tx.notify_success()?;
        Ok(())
    }
}

fn transition(config: &Config) -> Result<Transition> {
    Ok(Transition::new(config.pending())?
        .on_success(config.success())
        .on_failure(config.failure()))
}

#[cfg(test)]
mod test {
    use super::Blinc;
    use crate::config::Config;
    use crate::testutils::{init_logger, ReaderStub};
    use std::env;
    use std::fs::remove_file;
    use std::path::Path;
    use std::time::SystemTime;

    #[test]
    fn test_env_variables_are_set() {
        let config_content = r#"
            [[task]]
            cmd = "echo"
            args = [""]

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"

            [env]
            API_KEY = "10"
        "#
        .to_string();
        let config = Config::read(&mut ReaderStub::new(config_content)).unwrap();
        Blinc::new(config);
        assert_eq!(env::var("API_KEY").unwrap(), "10");
    }

    #[test]
    fn test_tasks_are_executed() {
        init_logger();
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let config_content = format!(
            r#"
            [[task]]
            cmd = "touch"
            args = ["/tmp/cargo-blinc-test-{timestamp}"]

            [colors]
            pending = ["blue", "blank"]
            failure = "red"
            success = "green"
        "#
        );
        let config = Config::read(&mut ReaderStub::new(config_content)).unwrap();
        let blinc = Blinc::new(config);
        blinc.exec_tasks().unwrap();
        assert!(Path::new(&format!("/tmp/cargo-blinc-test-{timestamp}")).exists());
        remove_file(format!("/tmp/cargo-blinc-test-{timestamp}")).unwrap();
    }
}
