#[macro_use]
extern crate lazy_static;
use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use std::boxed::Box;
use std::error::Error;
use std::time::Duration;

lazy_static! {
    static ref LONG_DURATION: Duration = Duration::from_millis(500);
    static ref BLUE: Color = Color::from("blue");
    static ref FADE_TO_BLUE_MSG: Message = Message::Fade(Color::from("blue"), *LONG_DURATION);
    static ref FADE_TO_WHITE_MSG: Message = Message::Fade(Color::from("white"), *LONG_DURATION);
}

fn main() -> Result<(), Box<dyn Error>> {
    let blinkers: Blinkers = match Blinkers::new() {
        Ok(b) => b,
        Err(_e) => {
            println!("unable to find device");
            return Ok(());
        }
    };

    blink_blue(&blinkers)?;
    blink_blue(&blinkers)?;
    blink_blue(&blinkers)?;

    Ok(())
}

fn blink_blue(blinkers: &Blinkers) -> Result<(), Box<dyn Error>> {
    blinkers.send(*FADE_TO_BLUE_MSG)?;
    std::thread::sleep(*LONG_DURATION);
    blinkers.send(*FADE_TO_WHITE_MSG)?;
    std::thread::sleep(*LONG_DURATION);
    Ok(())
}
