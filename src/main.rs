use std::boxed::Box;
use std::error::Error;
use transition::Transition;

mod testrunner;
mod transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("red blue green red blue")
        .on_success("green")
        .on_failure("red");
    transition.go();
    Ok(())
}
