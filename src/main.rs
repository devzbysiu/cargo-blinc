use std::boxed::Box;
use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::Transition;

mod testrunner;
mod transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("red blue green red blue")
        .on_success("green")
        .on_failure("red");
    transition.go();
    thread::sleep(Duration::from_secs(4));
    Ok(())
}
