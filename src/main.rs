use std::boxed::Box;
use std::error::Error;
use transition::Transition;

mod transition;

fn main() -> Result<(), Box<dyn Error>> {
    let transition = Transition::from("red blue green red blue green");
    transition.go()?;

    Ok(())
}
