use std::process::Command;
use std::process::ExitStatus;
use transition::Transition;

fn main() -> Result<(), failure::Error> {
    let tx = Transition::from("blue white")
        .on_success("green")
        .on_failure("red")
        .run()?;
    if run_tests()?.success() {
        tx.notify_success()?;
    } else {
        tx.notify_failure()?;
    }
    Ok(())
}

fn run_tests() -> Result<ExitStatus, failure::Error> {
    Ok(Command::new("cargo").arg("test").status()?)
}
