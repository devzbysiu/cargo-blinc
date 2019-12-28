use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_command_without_arguments() -> Result<(), failure::Error> {
    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.assert().success();
    Ok(())
}

#[test]
fn test_command_with_wrong_arguments() -> Result<(), failure::Error> {
    let mut cmd = Command::cargo_bin("cargo-blinc")?;
    cmd.arg("--init");
    cmd.assert().success();
    Ok(())
}
