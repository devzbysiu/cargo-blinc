use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

#[test]
fn test_command_without_arguments() -> Result<(), failure::Error> {
    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.assert().success();
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
