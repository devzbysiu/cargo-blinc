use assert_cmd::prelude::*;
use predicates::str::contains;
use serial_test::serial;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

#[test]
#[serial]
fn test_help_message() {
    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc");
    cmd.arg("--help");

    cmd.assert().success().stdout(contains(
        "-i, --init       Initializes configuration file named .blinc (note the dot)",
    ));
}

#[test]
#[serial]
fn test_command_without_arguments() {
    create_config(
        r#"
        [[tasks]]
        cmd = "cargo"
        args = ["check"]

        [colors]
        pending = ["blue", "white"]
        failure = "red"
        success = "green"
        "#,
        ".blinc",
    );

    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc").assert().success();

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
fn test_command_with_specified_path_to_config() {
    create_config(
        r#"
        [[tasks]]
        cmd = "cargo"
        args = ["check"]

        [colors]
        pending = ["blue", "white"]
        failure = "red"
        success = "green"
        "#,
        ".blinc-config",
    );

    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc")
        .arg("--config")
        .arg(".blinc-config")
        .assert()
        .success();

    fs::remove_file(".blinc-config").unwrap();
}

#[test]
#[serial]
fn test_config_init() {
    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc").arg("--init").assert().success();

    assert_eq!(
        read_config(".blinc"),
        r#"[[tasks]]
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
    );
    fs::remove_file(".blinc").unwrap();
}

#[test]
#[serial]
fn test_config_init_when_file_already_exists() {
    create_config(
        r#"
        [[tasks]]
        cmd = "cargo"
        args = ["check"]
        "#,
        ".blinc",
    );
    let mut cmd = Command::cargo_bin("cargo-blinc").unwrap();
    cmd.arg("blinc").arg("--init").assert().success();

    assert_eq!(
        read_config(".blinc"),
        r#"[[tasks]]
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
    );

    fs::remove_file(".blinc").unwrap();
}

fn create_config<I: Into<String>, A: AsRef<Path>>(config_content: I, path: A) {
    let config_content = config_content.into();
    let config_content: String = config_content.replace("\t", "");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.as_ref())
        .unwrap();
    file.write_all(config_content.as_bytes()).unwrap();
}

fn read_config<A: AsRef<Path>>(path: A) -> String {
    let mut config_content = String::new();
    File::open(path.as_ref())
        .and_then(|mut file| file.read_to_string(&mut config_content))
        .unwrap();
    config_content
}
