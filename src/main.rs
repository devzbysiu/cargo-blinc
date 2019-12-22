use std::boxed::Box;
use std::error::Error;
use std::process::Command;
use std::process::ExitStatus;
use std::time::Duration;
use transition::Msg;
use transition::Transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("blue white")
        .on_success("green")
        .on_failure("red");
    let sender = transition.go()?;
    if run_tests()?.success() {
        sender.send(Msg::Success)?;
    } else {
        sender.send(Msg::Failure)?;
    }
    std::thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn run_tests() -> Result<ExitStatus, failure::Error> {
    Ok(Command::new("cargo").arg("test").status()?)
}
