use std::boxed::Box;
use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::Msg;
use transition::Transition;

mod testrunner;
mod transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("red blue green red blue")
        .on_success("green")
        .on_failure("red");
    let sender = transition.go();
    thread::sleep(Duration::from_secs(10));
    sender.send(Msg::Success)?;
    Ok(())
}
