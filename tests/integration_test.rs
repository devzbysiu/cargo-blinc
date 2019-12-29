use assert_cmd::prelude::*;
use predicates::str::contains;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

#[test]
fn test_help_message() -> Result<(), failure::Error> {
    Ok(())
}

#[test]
fn test_command_without_arguments() -> Result<(), failure::Error> {
    let config_content = r#"[[tasks]]
cmd = "cargo"
args = ["check"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
"#;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(".blinc")?;
    file.write_all(config_content.as_bytes())?;

    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.arg("blinc");
    cmd.assert().success();

    fs::remove_file(".blinc")?;
    Ok(())
}

#[test]
fn test_command_with_invalid_argument() -> Result<(), failure::Error> {
    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.arg("--invalid");
    cmd.assert()
        .failure()
        .stderr(contains("USAGE"))
        .stderr(contains(
        "error: Found argument '--invalid' which wasn't expected, or isn't valid in this context",
    ));
    Ok(())
}

#[test]
fn test_config_init() -> Result<(), failure::Error> {
    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.arg("blinc").arg("--init").assert().success();

    let mut config_content = String::new();
    File::open(".blinc").and_then(|mut file| file.read_to_string(&mut config_content))?;
    assert_eq!(
        config_content,
        r#"[[tasks]]
cmd = "cargo"
args = ["check"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
"#
    );
    fs::remove_file(".blinc")?;
    Ok(())
}

#[test]
fn test_config_init_when_file_already_exists() -> Result<(), failure::Error> {
    let config_content = r#"[[tasks]]
cmd = "cargo"
args = ["check"]
"#;
    File::create(".blinc").and_then(|mut file| file.write_all(config_content.as_bytes()))?;

    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.arg("blinc").arg("--init").assert().success();

    let mut config_content = String::new();
    File::open(".blinc").and_then(|mut file| file.read_to_string(&mut config_content))?;
    assert_eq!(
        config_content,
        r#"[[tasks]]
cmd = "cargo"
args = ["check"]

[colors]
pending = ["blue", "white"]
failure = "red"
success = "green"
"#
    );

    fs::remove_file(".blinc")?;
    Ok(())
}
