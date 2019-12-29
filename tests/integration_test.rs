#[macro_use]
extern crate serial_test;

use assert_cmd::prelude::*;
use predicates::str::contains;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

#[test]
#[serial]
fn test_help_message() {}

#[test]
#[serial]
fn test_command_without_arguments() {
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
        .open(".blinc")
        .unwrap();
    file.write_all(config_content.as_bytes());

    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc");
    cmd.assert().success();

    fs::remove_file(".blinc").unwrap();
}

#[test]
#[serial]
fn test_command_with_invalid_argument() {
    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("--invalid");
    cmd.assert()
        .failure()
        .stderr(contains("USAGE"))
        .stderr(contains(
        "error: Found argument '--invalid' which wasn't expected, or isn't valid in this context",
    ));
}

#[test]
#[serial]
fn test_config_init() {
    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc").arg("--init").assert().success();

    let mut config_content = String::new();
    File::open(".blinc")
        .and_then(|mut file| file.read_to_string(&mut config_content))
        .unwrap();
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
    fs::remove_file(".blinc").unwrap();
}

#[test]
#[serial]
fn test_config_init_when_file_already_exists() {
    let config_content = r#"[[tasks]]
cmd = "cargo"
args = ["check"]
"#;
    File::create(".blinc")
        .and_then(|mut file| file.write_all(config_content.as_bytes()))
        .unwrap();

    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc").arg("--init").assert().success();

    let mut config_content = String::new();
    File::open(".blinc")
        .and_then(|mut file| file.read_to_string(&mut config_content))
        .unwrap();
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

    fs::remove_file(".blinc").unwrap();
}
