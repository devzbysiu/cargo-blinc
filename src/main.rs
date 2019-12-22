use std::boxed::Box;
use std::error::Error;
use std::time::Duration;
use testrunner::run_tests;
use transition::Msg;
use transition::Transition;

mod testrunner;
mod transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("red blue green red blue")
        .on_success("green")
        .on_failure("red");
    let sender = transition.go()?;
    std::thread::sleep(Duration::from_secs(5));
    if run_tests()?.success() {
        println!("success");
        sender.send(Msg::Success)?;
    } else {
        println!("failure");
        sender.send(Msg::Failure)?;
    }
    std::thread::sleep(Duration::from_secs(2));
    Ok(())
}
