use std::boxed::Box;
use std::error::Error;
use std::time::Duration;
use testrunner::run_tests;
use transition::Transition;

mod testrunner;
mod transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("red blue green red blue");
    transition.go()?;
    let green = Transition::from("green");
    let red = Transition::from("red");
    std::thread::sleep(Duration::from_millis(1000));
    match run_tests()?.success() {
        true => green.go()?,
        false => red.go()?,
    }
    Ok(())
}
