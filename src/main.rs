use std::boxed::Box;
use std::error::Error;
use std::time::Duration;
use testrunner::run_tests;
use transition::Msg;
use transition::Transition;

mod testrunner;
mod transition;

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
